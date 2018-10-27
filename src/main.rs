extern crate crypto;
extern crate crossbeam;


mod jacopone;
mod test;

use std::time::{Instant};
use std::fs::File;	
use std::io::prelude::*;
use std::env;
use jacopone::*;

fn main() {

	let args: Vec<_> = env::args().collect();
	if args.len() == 6 {
		let message = get_text_from_file(&args[1]);
		let key = args[2].as_bytes().clone().to_vec();
		let nonce = args[3].as_bytes().clone().to_vec();
		let counter = u64::from_str_radix(&args[4], 10).unwrap();
		let now = Instant::now(); 
		let threads  = u8::from_str_radix(&args[5], 10).unwrap();

		let data = CipherData::new(message, key, nonce, counter);
        let jacopone = jacopone::Jacopone::new(threads);
        let ciphertext = jacopone.encrypt(data);
        println!("{:?}",now.elapsed().as_secs() as f64
           + now.elapsed().subsec_nanos() as f64 * 1e-9 );
		write_to_file(&args[1], &ciphertext);


		
	}else{
		println!("usage: <filename> <key> <nonce> <counter> <number_of_threads>");
		println!("{:?}", args.len());
	}

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

