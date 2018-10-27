use super::utils::*;
use super::super::cipherdata::*;

pub fn jacopone_encrypt_ctr(data: CipherData) -> Vec<u8> {
    let mut c = data.counter;
    let mut ciphertext = Vec::new();
    for i in 0..data.message.len()/64 {
        let block_counter = get_block_counter(&data.nonce, & mut c);
        ciphertext.extend_from_slice(&xor(&block_encrypt(&block_counter, &data.key), &data.message[64 * i.. 64 * i + 64]));
    }
    let block_counter = get_block_counter(&data.nonce, & mut c);
    ciphertext.extend_from_slice(&xor(&data.message[(data.message.len()/64) * 64..], &block_encrypt(&block_counter, &data.key)));
    ciphertext
}

pub fn block_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ciphertext = message.clone().to_vec();
    for _i in 0..4 {
        ciphertext = feistel_round(&ciphertext, key);
        ciphertext = swap(&ciphertext);
    }
    ciphertext
} 

pub fn feistel_round(block: &[u8], key: &[u8]) -> Vec<u8> {
    let l = &block[0..32];
    let r = &block[32..];
    let mut l = xor(l, &hash(r, key));
    l.extend_from_slice(r);
    l
}



pub fn get_block_counter(nonce: &[u8], counter: & mut u64) -> Vec<u8> {
    let mut n = nonce.clone().to_vec();
    n.extend_from_slice(&(to_bytes(*counter)));
    *counter = (*counter).wrapping_add(1);
    n  
}


pub fn get_thread_blocks(message_len: usize, thread_count: u8) -> Vec<[u64; 2]>{
    let message_len = message_len as u64;
    let mut partition = Vec::new();
    let mut blocks_index = Vec::new(); 
    let block_num = message_len / 64;
    //if block_num / thread_count  as u64 > 0 {
    for _i in 0..thread_count {
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

    for i in 0..blocks_index.len() {
        if blocks_index[i][0] == blocks_index[i][1] {
            blocks_index.truncate(i);
            break;
        }
    }

    blocks_index
}