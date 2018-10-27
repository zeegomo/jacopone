mod utils;
mod thread;
use self::utils::*;
use self::thread::{ParallelThread, FinalThread};
use self::thread::parallelinterface::ParallelInterface;
use std::sync::Arc;

pub struct Jacopone{
    parallel_threads: ParallelThread,
}

impl Jacopone {

    //create a jacopone enviroment to encrypt/decrypt usong thread_count threads
    pub fn new(thread_count: u8) -> Jacopone {
        Jacopone {parallel_threads: thread::ParallelThread::new(thread_count)}
    }

    pub fn encrypt(&self, message: Arc<Vec<u8>>, key: Arc<Vec<u8>>, nonce: Arc<Vec<u8>>, counter: u64) -> Vec<u8> {
        assert_eq!(nonce.len(), 60, "invalid nonce len: {}. required: {}", nonce.len(), 60);
        //let cipher_data = CipherData {message: Arc::clone(&message), key: Arc::clone(&key), nonce: Arc::clone(&nonce), counter: counter};

        let cipherdata = CipherData{message:  Arc::clone(&message), key: Arc::clone(&key), nonce: Arc::clone(&nonce), counter: counter};
        
        //parallel encryption/decryption
        let mut ciphertext = self.parallel_threads.encrypt(CipherData::clone(&cipherdata));
        
        //encryption/decryption of last portion
        let ending = FinalThread::finalize_encryption(cipherdata);
        ciphertext.extend_from_slice(&ending);
        ciphertext
    }
}