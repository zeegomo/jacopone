extern crate crypto;

mod jacopone;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use jacopone::*;

fn main() {
	let message = "ciao".as_bytes();
	let key = [10, 7, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44];
	let ciphertext = jacopone_encrypt(&message, &key);
	println!("ciphertext: {:?}", String::from_utf8_lossy(&ciphertext));
	let decrypted = jacopone_decrypt(&ciphertext, &key);
	println!("{}", String::from_utf8_lossy(&decrypted));
}

