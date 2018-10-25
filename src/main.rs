extern crate crypto;

mod jacopone;




use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use jacopone::*;

fn main() {
	let message = "1000000001001001010010010010101101010101".as_bytes();
	let mut nonce = Vec::new();
	for i in 0..56 {
		nonce.push(22);
	}
	let key = [10, 7, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44];
	let ciphertext = jacopone_encrypt_ctr(&message, &key, &nonce, 0);
	println!("ciphertext: {:?}", String::from_utf8_lossy(&ciphertext));
	let decrypted = jacopone_decrypt_ctr(&ciphertext, &key, &nonce, 0);
	println!("decrypted: {:?}", String::from_utf8_lossy(&decrypted));
    let m: u64 = 34;
    
}

