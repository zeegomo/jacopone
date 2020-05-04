#[macro_use]
extern crate clap;
extern crate jacopone;

use clap::{App, Arg, ErrorKind::ArgumentNotFound};
use jacopone::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod options;
use options::*;

macro_rules! get_arg_or_default {
    ($matc:ident, $str:expr, $expl:expr, $enum:ty) => {
        value_t!($matc.value_of($str), $enum)
            .unwrap_or_else(|e| {
                if e.kind != ArgumentNotFound {
                    panic!("invalid {} value", $expl);
                }

                <$enum>::default()
            })
            .to_jacopone();
    };
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .arg(
            Arg::with_name("mode")
                .help("specify the block cipher mode")
                .short("m")
                .takes_value(true)
                .possible_values(&JMode::variants()),
        )
        .arg(
            Arg::with_name("ks")
                .help("specify the key scheduling")
                .short("ks")
                .takes_value(true)
                .possible_values(&JScheduler::variants()),
        )
        .arg(
            Arg::with_name("padding")
                .help("specify the padding used")
                .short("p")
                .takes_value(true)
                .possible_values(&JPadding::variants()),
        )
        .arg(
            Arg::with_name("rfunction")
                .help("specify the round function")
                .short("f")
                .takes_value(true)
                .possible_values(&JFunction::variants()),
        )
        .get_matches();

    let output = matches
        .value_of("output")
        .unwrap_or(matches.value_of("INPUT").unwrap());

    let mut message = get_text_from_file(matches.value_of("INPUT").unwrap());
    let mut key = matches.value_of("KEY").unwrap().as_bytes().to_vec();
    key.resize(32, 0);

    let nonce_vec = matches
        .value_of("nonce")
        .map(|nonce| nonce.as_bytes().to_vec());

    let nonce = if let Some(ref vec) = nonce_vec {
        Some(vec as &[u8])
    } else {
        None
    };

    let now = Instant::now();

    let function = get_arg_or_default!(matches, "rfunction", "round function", JFunction);
    let ks = get_arg_or_default!(matches, "ks", "key scheduler", JScheduler);
    let padding = get_arg_or_default!(matches, "padding", "padding", JPadding);
    let mode = get_arg_or_default!(matches, "mode", "mode", JMode);

    let jacopone = Jacopone::new(mode, function, ks, padding);

    if matches.is_present("DECRYPT") {
        println!("Decrypting {} bytes...", message.len());
        jacopone.decrypt(&mut message, &key, nonce);
    } else {
        println!("Encrypting {} bytes...", message.len());
        jacopone.encrypt(&mut message, &key, nonce);
    }

    println!(
        "Done: {:?}",
        now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9
    );

    write_to_file(output, &message);
}

fn write_to_file(filename: &str, content: &[u8]) {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    file.write_all(content).unwrap();
}

fn get_text_from_file(filename: &str) -> Vec<u8> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut text = Vec::new();
    file.read_to_end(&mut text).ok().expect("failed to read!");
    text
}
