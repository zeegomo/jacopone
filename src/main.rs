extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;

fn main() {
	let message = "ciao".as_bytes();
	let key = [10, 7, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44];
	let ciphertext = jacopone_encypt(&message, &key);
	println!("ciphertext: {:?}", String::from_utf8_lossy(&ciphertext));
	let decrypted = jacopone_decrypt(&ciphertext, &key);
	println!("{}", String::from_utf8_lossy(&decrypted));
}

fn jacopone_encypt(message: &[u8], key: &[u8]) -> Vec<u8> {
	let mut ciphertext = pad(message, 64);
	for _i in 0..4 { 
		ciphertext = feistel_round(&ciphertext, key);
		ciphertext = swap(&ciphertext);
	}
	ciphertext
}

fn feistel_round(block: &[u8], key: &[u8]) -> Vec<u8> {
	let l = &block[0..32];
	let r = &block[32..];
	let mut l = xor(l, &hash(r, key));
	l.extend_from_slice(r);
	l
}

fn swap(message: &[u8]) -> Vec<u8>{
	let l = &message[0..32];
	let mut r = (&message[32..]).to_vec();
	r.extend_from_slice(l);
	r
}

fn jacopone_decrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
	let mut ciphertext = message.clone().to_vec();
	for _i in 0..4 {
		ciphertext = swap(&ciphertext); 
		ciphertext = feistel_round(&ciphertext, key);
	}
	ciphertext
}

fn xor(s1: &[u8], s2: &[u8]) -> Vec<u8> {
	s1.iter().zip(s2).map(|(x, y)| x ^ y).collect()
}

fn hex_to_bytes(string: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..(string.len() / 2) {
        match u8::from_str_radix(&string[2 * i..2 * i + 2], 16) {
            Ok(n) => bytes.push(n),
            Err(_) => println!("Error in hex conversion"),
        }
    }
    bytes
}

pub fn pad(text: &[u8], length: u8) -> Vec<u8> {
    let mut padded = Vec::new();
    for i in 0..text.len(){
    	padded.push(text[i]);
    }
    let mut i: i32 = 1;
    while (length as i32 * i - text.len() as i32) < 0 {
        i += 1;
    }
    let padding = (length as u32 * i as u32 - text.len() as u32) as u8;
    for _i in text.len()..(padding as usize + text.len()) {
        padded.push(padding);
    }

    padded
}

fn hash(block: &[u8], key: &[u8]) -> Vec<u8>{
	let mut hasher = Sha3::sha3_256();
	hasher.input(&xor(block,key));
	hex_to_bytes(&hasher.result_str())
}
