extern crate clap;
extern crate hex;
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
    let value = &{
        if let Some(v) = matches.values_of("value") {
            v.collect::<Vec<_>>().join(" ")
        } else {
            // TODO read all available data from stdin
            todo!()
        }
    };

    decode_encode(from, to, _as, value);
}

fn decode_encode(from: &str, to: Vec<&str>, _as: &str, value: &str) {
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

    // TODO if tty
    if stdout_isatty() {
        println!(
            "\t[{} ~> {}]\n",
            from_format,
            to_formats
                .clone()
                .into_iter()
                .map(|v| v.to_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    match decode(from_format, value) {
        Ok(data) => {
            //
            to_formats.into_iter().for_each(|format| {
                // TODO different format when TTY
                println!("{}: \"{}\"", format.to_str(), encode(format, data.clone()));
            })
        }
        Err(e) => panic!("couldn't decode! {:?}", e),
    }
}

fn decode(f: Format, value: &str) -> Result<Vec<u8>, Error> {
    match f {
        Format::Hex => codecs::hex::HexCodec::decode(value),
        Format::Ascii => codecs::ascii::AsciiCodec::decode(value),
        Format::Base64 => codecs::base64::Base64Codec::decode(value),
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

fn infer(data: &str) -> Result<Vec<u8>, Error> {
    // TODO
    if let Ok(v) = codecs::hex::HexCodec::decode(data) {
        return Ok(v);
    }
    if let Ok(v) = codecs::base64::Base64Codec::decode(data) {
        return Ok(v);
    }
    // instead, always fall back to raw bytes TODO
    return codecs::ascii::AsciiCodec::decode(data);
}
