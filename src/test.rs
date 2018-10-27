#[cfg(test)]
mod tests {
	extern crate rand;
    use super::*;
    use jacopone::*;
    use self::rand::prelude::*;
    use self::rand::Rng;

	use std::fs::File;	
	use std::io::prelude::*;

    #[test]
    fn check_decrypt() {
    	let mut rng = thread_rng();
        let jacopone = Jacopone::new(4);
    	let mut message = lines_from_file("/home/zeegomo/Documents/crypto/ciphers/jacopone/src/strings.txt");
		for i in 0..message.len() {
			let mut nonce: Vec<u8> = Vec::new();
			for j in 0..60 {
				nonce.push(rng.gen());
			}
			let key = vec![10, 7, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44, 21, 33, 32, 76, 54, 45, 12, 87, 09, 12, 43, 87, 43 ,23, 44];
			let counter: u64 = rand::random::<u64>();
            let data = CipherData::new((&message[i]).as_bytes().to_vec(), &key, &nonce, counter);
			let ciphertext = jacopone.encrypt(data);
            let data = CipherData::new(ciphertext, &key, &nonce, counter);
            
			let plaintext = jacopone.encrypt(data);

			//println!("{} {}", (&message[i]).as_bytes().len(), plaintext.len());

        	assert_eq!(&message[i].as_bytes().to_vec(), &plaintext);
    	}
    }

    fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
	}
	fn hex_to_bytes(string: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in 0..(string.len() / 2) {
        match u8::from_str_radix(&string[2 * i.. 2 * i + 2], 16) {
            Ok(n) => bytes.push(n),
            Err(_) => panic!("Error in hex conversion"),
        }
    }
    bytes
	}
}