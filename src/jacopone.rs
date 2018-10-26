use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::sync::mpsc;

pub fn jacopone_encrypt_ctr_threaded(message: &[u8], key: &[u8], nonce: &[u8], counter: u64, thread_count: u8) -> Vec<u8> {
    //check key, counter and nonce length
    if nonce.len() != 60 {
        println!("{:?}", nonce.len());
        panic!{"invalid nonce length"};
    }

    if thread_count > 8 {
        panic!("invalid thread number: max = 8");
    }

    let mut txv = Vec::new();
    let mut rxv = Vec::new();
    
    //create transmitter (tx) and receiver (rx) for each thread
    for i in 0..thread_count {
        let (tx1, rx1) = mpsc::channel();
        txv.push(tx1);
        rxv.push(rx1);
    }

    let blocks_index = get_thread_blocks(message.len(), thread_count);
    //spawnw thread_count threads
    crossbeam::scope(|scope|{
        for i in 0..thread_count as usize {
            let tx = mpsc::Sender::clone(&txv[i]);
            let start = blocks_index[i][0] as usize;
            let end = blocks_index[i][1] as usize;
            if end - start > 0 {
                scope.spawn(move ||{
                    let mut c = counter + start as u64;
                    let mut ciphertext = jacopone_encrypt_ctr(&message[start * 64 .. end * 64], key, nonce, c);
                    tx.send(ciphertext).unwrap();
                    });
                }      
            }
    });

    //receive results from threads
    let mut blocks = Vec::new();
    for i in 0..thread_count as usize {
        if blocks_index[i][1] - blocks_index[i][0] > 0 {
            blocks.push(rxv[i].recv().unwrap()); 
        }
    }
    let mut ciphertext = Vec::new();
    for i in 0..thread_count as usize {
        if i < blocks.len() {
            ciphertext.extend_from_slice(&blocks[i]);
        }
    }

    //encryt (or decrypt) the remaining bytes
    let mut c = counter + (blocks_index[blocks_index.len() -1][1]) as u64;
    let block_counter = get_block_counter(nonce, & mut c);
    let ending = xor(&message[blocks_index[blocks_index.len() -1][1] as usize * 64..], &block_encrypt(&block_counter, key));
    ciphertext.extend_from_slice(&ending);
    ciphertext
}

pub fn get_thread_blocks(message_len: usize, thread_count: u8) -> Vec<[u64; 2]>{
    let message_len = message_len as u64;
    let mut partition = Vec::new();
    let mut blocks_index = Vec::new(); 
    println!("{:?}", message_len);
    let block_num = message_len / 64;
    println!("{:?}", block_num);
    //if block_num / thread_count  as u64 > 0 {
        for i in 0..thread_count {
            partition.push(block_num / thread_count as u64);
        }
    
    let mut res = block_num - (block_num / thread_count as u64) * thread_count as u64;
    for i in 0..thread_count as usize {
        if res > 0 {
            partition[i] = partition[i] + 1;
            res = res - 1;
        }
    }
    
    blocks_index.push([0, partition[0]]);
    let mut last = partition[0];
    for i in 1..thread_count as usize {
        blocks_index.push([last, last + partition[i]]);
        last = last + partition[i];
    }

    blocks_index
}

pub fn jacopone_encrypt_ctr(message: &[u8], key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
    //check key, counter and nonc
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