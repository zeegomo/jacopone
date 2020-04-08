#[macro_use]
extern crate clap;
extern crate jacopone;

use clap::App;
use jacopone::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut message = get_text_from_file(matches.value_of("INPUT").unwrap());
    let mut key = matches.value_of("KEY").unwrap().as_bytes().to_vec();
    key.resize(32, 0);
    let nonce = matches.value_of("NONCE").unwrap().as_bytes().to_vec();

    let output = matches
        .value_of("output")
        .unwrap_or(matches.value_of("INPUT").unwrap());

    let now = Instant::now();
    println!("Encrypting {} bytes...", message.len());
    let jacopone = Jacopone::<ModeCTR, Sha2, Dummy>::new();
    jacopone.encrypt(&mut message, &key, &nonce);

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
