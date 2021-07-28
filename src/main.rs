extern crate clap;
extern crate isatty;

mod codecs;
mod options;

use clap::{App, Arg};
use codecs::codec::Codec;
use codecs::error::Error;
use isatty::stdout_isatty;
use options::Format;
use std::ffi::OsString;
use std::io;
use std::io::Read;
use std::process;

// Windows uses UTF-16. We'll convert it to UTF-8 and give the bytes.
#[cfg(target_os = "windows")]
fn u8_arg(s: &OsString) -> Result<Vec<u8>, Error> {
    match s.to_str() {
        Some(s) => Ok(s.bytes().collect::<Vec<u8>>()),
        None => Err(Error::new(
            "Only UTF input supported on Windows via CLI argument.".to_string(),
        )),
    }
}

// Linux can have anything in this string. We'll just return the bytes.
#[cfg(not(target_os = "windows"))]
fn u8_arg(s: &OsString) -> Result<Vec<u8>, Error> {
    // Gives OsString the into_vec() method
    use std::os::unix::ffi::OsStringExt;
    Ok(s.to_owned().into_vec())
}

fn main() {
    let matches = App::new("rc")
        .version("1.0")
        .author("Elliot Cubit")
        .about("Converts between encodings and number bases")
        .arg(
            Arg::new("from")
                .about("The encoding or base to convert from")
                .takes_value(true)
                .value_name("format")
                .short('f')
                .long("from")
                .possible_values(&Format::all_variants()),
        )
        .arg(
            Arg::new("to")
                .about("The encoding or base to convert to")
                .takes_value(true)
                .value_name("format")
                .short('t')
                .long("to")
                // Allow multiple values, but require explicit flag
                // for each value so we can respect positional args
                .number_of_values(1)
                .multiple(true)
                .possible_values(&Format::all_variants()),
        )
        .arg(
            Arg::new("as")
                .about("How to display the output")
                .takes_value(true)
                .value_name("format")
                .short('a')
                .long("as")
                .default_value("text"),
        )
        .arg(
            Arg::new("verbose")
                .about("Provide more verbose output")
                .short('v')
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("value")
                .about("What to convert")
                .value_name("val")
                .index(1)
                .multiple(true),
        )
        .get_matches();

    let from = matches.value_of("from").unwrap_or_else(|| "__infer");
    let to = {
        if let Some(v) = matches.values_of("to") {
            v.collect::<Vec<_>>()
        } else {
            vec!["utf8", "hex", "base64"]
        }
    };
    let _as = matches.value_of("as").unwrap_or_else(|| "text");
    let verbosity = matches.occurrences_of("verbose");
    let value = &{
        if let Some(v) = matches.values_of_os("value") {
            match v
                .map(|v| v.to_owned())
                .collect::<Vec<OsString>>()
                .iter()
                .map(u8_arg)
                .collect::<Result<Vec<Vec<u8>>, Error>>()
                .map(|o| {
                    o.into_iter()
                        .reduce(|sum, cur| vec![sum, vec![0x20], cur].concat())
                        .unwrap()
                }) {
                Ok(fin) => fin,
                // TODO clean up
                Err(e) => {
                    println!("Error parsing CLI arguments: {}", e.err);
                    process::exit(1)
                }
            }
        } else {
            // TODO read all available data from stdin
            // Needs mild refactor for normal values to be Vec<u8> not String
            let mut v: Vec<u8> = vec![];
            io::stdin().lock().read_to_end(&mut v).unwrap();
            v
        }
    };

    decode_encode(from, to, _as, verbosity, value.to_vec());
}

fn decode_encode(from: &str, to: Vec<&str>, _as: &str, verbosity: u64, value: Vec<u8>) {
    // These unwrap()s are safe since the argument parser validates these values exist
    let from_format = Format::from_str(from).unwrap();
    // TODO dedupe output formats
    let to_formats = to
        .into_iter()
        .map(|v| Format::from_str(v).unwrap())
        .collect::<Vec<_>>();

    match decode(&from_format, value) {
        (used_format, Ok(data)) => {
            if stdout_isatty() && (from_format == Format::Inferred || verbosity > 0) {
                println!(
                    "\t[{}{}~> {}]\n",
                    used_format,
                    if used_format != from_format {
                        " (inferred) "
                    } else {
                        " "
                    },
                    to_formats
                        .clone()
                        .into_iter()
                        .map(|v| v.to_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            let do_leader = (verbosity > 0 && stdout_isatty()) || to_formats.len() > 1;
            to_formats.into_iter().for_each(|format| {
                if do_leader {
                    println!("{}: \"{}\"", format.to_str(), encode(format, data.clone()));
                } else {
                    // No newline if we're piping a single format
                    print!("{}", encode(format, data.clone()))
                }
            })
        }
        (_, Err(e)) => {
            println!("Couldn't decode! {}", e.err);
            process::exit(1)
        }
    }
}

fn decode(f: &Format, value: Vec<u8>) -> (Format, Result<Vec<u8>, Error>) {
    match f {
        Format::Hex => (Format::Hex, codecs::hex::HexCodec::decode(value)),
        Format::Utf8 => (Format::Utf8, codecs::utf8::Utf8Codec::decode(value)),
        Format::Base64 => (Format::Base64, codecs::base64::Base64Codec::decode(value)),
        Format::Raw => (Format::Raw, codecs::raw::RawCodec::decode(value)),
        Format::Inferred => infer(value),
        _ => todo!(),
    }
}

fn encode(f: Format, data: Vec<u8>) -> String {
    match f {
        Format::Hex => codecs::hex::HexCodec::encode(data),
        Format::Utf8 => codecs::utf8::Utf8Codec::encode(data),
        Format::Base64 => codecs::base64::Base64Codec::encode(data),
        Format::Raw => codecs::raw::RawCodec::encode(data),
        _ => todo!(),
    }
}

fn infer(data: Vec<u8>) -> (Format, Result<Vec<u8>, Error>) {
    if let Ok(v) = codecs::hex::HexCodec::decode(data.clone()) {
        return (Format::Hex, Ok(v));
    }
    if let Ok(v) = codecs::base64::Base64Codec::decode(data.clone()) {
        return (Format::Base64, Ok(v));
    }
    if let Ok(v) = codecs::utf8::Utf8Codec::decode(data.clone()) {
        return (Format::Utf8, Ok(v));
    }
    // TODO raw bytes as the fallback
    return (Format::Raw, codecs::raw::RawCodec::decode(data.clone()));
}
