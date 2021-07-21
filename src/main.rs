extern crate clap;
extern crate isatty;

mod codecs;
mod options;

use clap::{App, Arg};
use codecs::codec::Codec;
use codecs::error::Error;
use isatty::stdout_isatty;
use options::Format;

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
                .default_value("__infer")
                .hide_default_value(true)
                .possible_values(&Format::all_variants()),
        )
        .arg(
            Arg::new("to")
                .about("The encoding or base to convert to")
                .takes_value(true)
                .value_name("format")
                .short('t')
                .long("to")
                .default_value("__infer")
                .hide_default_value(true)
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
            vec!["__infer"]
        }
    };
    let _as = matches.value_of("as").unwrap_or_else(|| "text");
    let verbosity = matches.occurrences_of("verbose");
    let value = &{
        if let Some(v) = matches.values_of("value") {
            v.collect::<Vec<_>>().join(" ")
        } else {
            // TODO read all available data from stdin
            // Needs mild refactor for normal values to be Vec<u8> not String
            todo!()
        }
    };

    decode_encode(from, to, _as, verbosity, value);
}

fn decode_encode(from: &str, to: Vec<&str>, _as: &str, verbosity: u64, value: &str) {
    // These unwrap()s are safe since the argument parser validates these values exist
    let from_format = Format::from_str(from).unwrap();
    // TODO dedupe output formats
    let to_formats = to
        .into_iter()
        .flat_map(|v| {
            let as_format = Format::from_str(v).unwrap();
            if as_format == Format::Inferred {
                // Default output formats
                vec![Format::Ascii, Format::Hex, Format::Base64].into_iter()
            } else {
                // Or use any that we were provided
                vec![as_format].into_iter()
            }
        })
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
            let do_leader = stdout_isatty() || to_formats.len() > 1;
            to_formats.into_iter().for_each(|format| {
                if do_leader {
                    println!("{}: \"{}\"", format.to_str(), encode(format, data.clone()));
                } else {
                    // No newline if we're piping a single format
                    print!("{}", encode(format, data.clone()))
                }
            })
        }
        (_, Err(e)) => panic!("couldn't decode! {:?}", e),
    }
}

fn decode(f: &Format, value: &str) -> (Format, Result<Vec<u8>, Error>) {
    match f {
        Format::Hex => (Format::Hex, codecs::hex::HexCodec::decode(value)),
        Format::Ascii => (Format::Hex, codecs::ascii::AsciiCodec::decode(value)),
        Format::Base64 => (Format::Hex, codecs::base64::Base64Codec::decode(value)),
        Format::Inferred => infer(value),
        _ => todo!(),
    }
}

fn encode(f: Format, data: Vec<u8>) -> String {
    match f {
        Format::Hex => codecs::hex::HexCodec::encode(data),
        Format::Ascii => codecs::ascii::AsciiCodec::encode(data),
        Format::Base64 => codecs::base64::Base64Codec::encode(data),
        _ => todo!(),
    }
}

fn infer(data: &str) -> (Format, Result<Vec<u8>, Error>) {
    // TODO
    if let Ok(v) = codecs::hex::HexCodec::decode(data) {
        return (Format::Hex, Ok(v));
    }
    if let Ok(v) = codecs::base64::Base64Codec::decode(data) {
        return (Format::Base64, Ok(v));
    }
    // instead, always fall back to raw bytes TODO
    return (Format::Ascii, codecs::ascii::AsciiCodec::decode(data));
}
