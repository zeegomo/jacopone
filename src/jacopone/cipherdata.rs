use std::sync::Arc;

pub struct CipherData{
    pub message: Arc<Vec<u8>>,
    pub key: Arc<Vec<u8>>,
    pub nonce: Arc<Vec<u8>>,
    pub counter: u64,
}

impl CipherData {
    pub fn clone(other: &CipherData) -> CipherData {
        CipherData {message: Arc::clone(&other.message), key: Arc::clone(&other.key), nonce: Arc::clone(&other.nonce), counter: other.counter}
    }

    pub fn clone_slice(other: &CipherData, start: usize, end: usize) -> CipherData {
        CipherData {message: Arc::new(other.message[start * 64 .. end * 64].to_vec()), key: Arc::clone(&other.key), nonce: Arc::clone(&other.nonce), counter: other.counter + start as u64}
 
    }
}
