#[macro_use]
extern crate clap;
extern crate jacopone;

use std::time::{Instant};
use std::fs::File;
use std::io::prelude::*;	
use jacopone::*;
use clap::App;

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let message = get_text_from_file(matches.value_of("INPUT").unwrap());
    let key = matches.value_of("KEY").unwrap().as_bytes().to_vec();
    let nonce = matches.value_of("NONCE").unwrap().as_bytes().to_vec();
    let counter = u64::from_str_radix(matches.value_of("COUNTER").unwrap(), 10).unwrap();
    let threads = u8::from_str_radix(matches.value_of("threads").unwrap_or("2"), 10).unwrap();
    let output = matches.value_of("output").unwrap_or(matches.value_of("INPUT").unwrap());

    let now = Instant::now();

    let jacopone = Jacopone::new(threads);
    let data = CipherData::new(message, key, nonce, counter);
    let ciphertext = jacopone.encrypt(data);

    println!("elapsed: {:?}",now.elapsed().as_secs() as f64
           + now.elapsed().subsec_nanos() as f64 * 1e-9 );

    write_to_file(output, &ciphertext);

}

fn write_to_file(filename: &str, content: &[u8]){
	let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    file.write_all(content);
}

fn get_text_from_file(filename: &str) -> Vec<u8>{
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut text = Vec::new();
    file.read_to_end(&mut text)
        .ok()
        .expect("failed to read!");
    text
}


