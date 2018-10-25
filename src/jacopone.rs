use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::sync::mpsc;

pub fn jacopone_encrypt_ctr_threaded(message: &[u8], key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
    //check key, counter and nonce length
    if nonce.len() != 60 {
        println!("{:?}", nonce.len());
        panic!{"invalid nonce length"};
    }

    

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();
    let (tx4, rx4) = mpsc::channel();

    let txv = [tx1, tx2, tx3, tx4];    

    crossbeam::scope(|scope|{
        for i in 0..4 {
            let tx = txv[i].clone();
            scope.spawn(move ||{
                println!("core{} started", i);
                let mut c = counter;
                let mut ciphertext = Vec::new();
                for i in 0..(message.len()/64)/4 {
                    let block_counter = get_block_counter(&nonce, & mut c);
                    ciphertext.extend_from_slice(&xor(&block_encrypt(&block_counter, &key), &message[64 * i.. 64 * i + 64]));
                }
                println!("core{} finished", i);
                tx.send(ciphertext).unwrap();
                });
        }/*
        scope.spawn(move ||{
        println!("core1 started");
        let mut c = counter;
        let mut ciphertext = Vec::new();
        for i in 0..(message.len()/64)/4 {
            let block_counter = get_block_counter(&nonce, & mut c);
            ciphertext.extend_from_slice(&xor(&block_encrypt(&block_counter, &key), &message[64 * i.. 64 * i + 64]));
        }
        println!("core1 finished");
        tx1.send(ciphertext).unwrap();
        });

        scope.spawn(move ||{
        println!("core2 started");
        let mut c = counter + ((message.len()/64)/4) as u64;
        let mut ciphertext = Vec::new();
        for i in(message.len()/64)/4..(message.len()/64)/2 {
            let block_counter = get_block_counter(&nonce, & mut c);
            ciphertext.extend_from_slice(&xor(&block_encrypt(&block_counter, &key), &message[64 * i.. 64 * i + 64]));
        }
        println!("core2 finished");
        tx2.send(ciphertext).unwrap();
        });

        scope.spawn(move ||{
        println!("core3 started");
        let mut c = counter + ((message.len()/64)/2) as u64;
        let mut ciphertext = Vec::new();
        for i in(message.len()/64)/2..((message.len()/64)/4)*3 {
            let block_counter = get_block_counter(&nonce, & mut c);
            ciphertext.extend_from_slice(&xor(&block_encrypt(&block_counter, &key), &message[64 * i.. 64 * i + 64]));
        }
        println!("core3 finished");
        tx3.send(ciphertext).unwrap();
        });

        scope.spawn(move ||{
        println!("core4 started");
        let mut c = counter + (((message.len()/64)/4)*3) as u64;
        let mut ciphertext = Vec::new();
        for i in ((message.len()/64)/4)*3..(message.len()/64) {
            let block_counter = get_block_counter(&nonce, & mut c);
            ciphertext.extend_from_slice(&xor(&block_encrypt(&block_counter, &key), &message[64 * i.. 64 * i + 64]));
        }
        println!("core4 finished");
        tx4.send(ciphertext).unwrap();
        });*/

    });

    println!("core joined");
    let mut c = counter + (message.len()/64) as u64;
    let mut block1 = rx1.recv().unwrap();
    let block2 = rx2.recv().unwrap();
    let block3 = rx3.recv().unwrap();
    let block4 = rx4.recv().unwrap();
    let block_counter = get_block_counter(nonce, & mut c);
    let ending = xor(&message[(message.len()/64) * 64..], &block_encrypt(&block_counter, key));
    block1.extend_from_slice(&block2);
    block1.extend_from_slice(&block3);
    block1.extend_from_slice(&block4);
    block1.extend_from_slice(&ending);
    block1
}
pub fn jacopone_encrypt_ctr(message: &[u8], key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
    //check key, counter and nonce length
    let mut c = counter;
    let mut ciphertext = Vec::new();
    for i in 0..message.len()/64 {
        let block_counter = get_block_counter(nonce, & mut c);
        ciphertext.extend_from_slice(&xor(&block_encrypt(&block_counter, key), &message[64 * i.. 64 * i + 64]));
    }
    let block_counter = get_block_counter(nonce, & mut c);
    ciphertext.extend_from_slice(&xor(&message[(message.len()/64) * 64..], &block_encrypt(&block_counter, key)));
    ciphertext
}

fn block_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ciphertext = message.clone().to_vec();
    for _i in 0..4 {
        ciphertext = feistel_round(&ciphertext, key);
        ciphertext = swap(&ciphertext);
    }
    ciphertext
} 

/*fn block_decrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut plaintext = message.clone().to_vec();
    for _i in 0..4 {
        plaintext = swap(&plaintext);
        plaintext = feistel_round(&plaintext, key);
    }
    plaintext
}
*/ 
fn feistel_round(block: &[u8], key: &[u8]) -> Vec<u8> {
    let l = &block[0..32];
    let r = &block[32..];
    let mut l = xor(l, &hash(r, key));
    l.extend_from_slice(r);
    l
}

fn swap(block: &[u8]) -> Vec<u8> {
    let l = &block[0..32];
    let mut r = (&block[32..]).to_vec();
    r.extend_from_slice(l);
    r
}

pub fn xor(s1: &[u8], s2: &[u8]) -> Vec<u8> {
    s1.iter().zip(s2).map(|(x, y)| x ^ y).collect() 
}

/*fn pad(text: &[u8], length: u8) -> Vec<u8> {
    let mut padded = Vec::new();
    for i in 0..text.len() {
        padded.push(text[i]);
    }
    let mut i: i32 = 1;
    while (length as i32 * i - text.len() as i32) < 0 {
        i += 1;
    }
    let padding = (length as u32 * i as u32 - text.len() as u32) as u8;
    for _i in 0..padding as usize {
        padded.push(padding);
    }
    padded
}*/

fn hash(block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3::sha3_256();
    let mut round_block = block.to_vec();
    round_block.extend_from_slice(key);
    hasher.input(&round_block);
    hex_to_bytes(&hasher.result_str())
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

fn get_block_counter(nonce: &[u8], counter: & mut u64) -> Vec<u8> {
    let mut n = nonce.clone().to_vec();
    n.extend_from_slice(&(to_bytes(*counter)));
    *counter = (*counter).wrapping_add(1);
    n  
}

fn to_bytes(n: u64) -> Vec<u8> {
    let mut n = n;
    let mut v = Vec::new();
    for _i in 0..4{
        v.push((n & 255) as u8);
        n = n >> 8;
    }
    v
}