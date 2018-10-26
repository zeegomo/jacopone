use crypto::sha3::Sha3;
use crypto::digest::Digest;

pub fn to_bytes(n: u64) -> Vec<u8> {
    let mut n = n;
    let mut v = Vec::new();
    for _i in 0..4{
        v.push((n & 255) as u8);
        n = n >> 8;
    }
    v
}

pub fn hex_to_bytes(string: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in 0..(string.len() / 2) {
        match u8::from_str_radix(&string[2 * i.. 2 * i + 2], 16) {
            Ok(n) => bytes.push(n),
            Err(_) => panic!("Error in hex conversion"),
        }
    }
    bytes
}

pub fn xor(s1: &[u8], s2: &[u8]) -> Vec<u8> {
    s1.iter().zip(s2).map(|(x, y)| x ^ y).collect() 
}


pub fn swap(block: &[u8]) -> Vec<u8> {
    let l = &block[0..32];
    let mut r = (&block[32..]).to_vec();
    r.extend_from_slice(l);
    r
}

pub fn hash(block: &[u8], key: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3::sha3_256();
    let mut round_block = block.to_vec();
    round_block.extend_from_slice(key);
    hasher.input(&round_block);
    hex_to_bytes(&hasher.result_str())
}


pub fn get_block_counter(nonce: &[u8], counter: & mut u64) -> Vec<u8> {
    let mut n = nonce.clone().to_vec();
    n.extend_from_slice(&(to_bytes(*counter)));
    *counter = (*counter).wrapping_add(1);
    n  
}
