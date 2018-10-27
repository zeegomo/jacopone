mod thread;
pub mod cipherdata;

pub use self::cipherdata::*;
use self::thread::{ParallelThread, FinalThread};

pub struct Jacopone{
    parallel_threads: ParallelThread,
}

impl Jacopone {

    //create a jacopone enviroment to encrypt/decrypt usong thread_count threads
    pub fn new(thread_count: u8) -> Jacopone {
        Jacopone {parallel_threads: thread::ParallelThread::new(thread_count)}
    }

    pub fn encrypt(&self, data: CipherData) -> Vec<u8> {
        assert_eq!(data.nonce.len(), 60, "invalid nonce len: {}. required: {}", data.nonce.len(), 60);
        
        //parallel encryption/decryption
        let mut ciphertext = self.parallel_threads.encrypt(CipherData::clone(&data));
        
        //encryption/decryption of last portion
        let ending = FinalThread::finalize_encryption(data);
        ciphertext.extend_from_slice(&ending);
        ciphertext
    }
}