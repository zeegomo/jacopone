pub use super::utils::*;
use std::sync::Arc;
pub mod parallelinterface;



pub struct FinalThread {

}

impl FinalThread {
	pub fn finalize_encryption(message: Arc<Vec<u8>>, key: Arc<Vec<u8>>, nonce: Arc<Vec<u8>>, counter: u64) -> Vec<u8> {
		let mut c = counter + (message.len()/64) as u64;
        let block_counter = get_block_counter(&nonce, & mut c);
        xor(&message[message.len()/64 * 64..], &block_encrypt(&block_counter, &key))
	}
}


pub struct ParallelThread {
	thread_count: u8,
	parallel_interface: parallelinterface::ParallelInterface,
}

impl ParallelThread {

	pub fn new (n: u8) -> ParallelThread{
		let interface = parallelinterface::ParallelInterface::new(n);
		ParallelThread {thread_count: n,  parallel_interface: interface}
	}

	pub fn encrypt(&self, message: Arc<Vec<u8>>, key: Arc<Vec<u8>>, nonce: Arc<Vec<u8>>, counter: u64) -> Vec<u8> {
		/*let message = data.message;
		let key = data.key;
		let nonce = data.nonce;
		let counter = data.counter;*/

		let blocks_index = get_thread_blocks(message.len(), self.thread_count);

    	//spawnw thread_count threads
    	self.spawn_threads(message, key, nonce, counter, &blocks_index);
    	self.parallel_interface.concat(blocks_index.len() as u8)
	}

	fn spawn_threads(&self, message: Arc<Vec<u8>>, key: Arc<Vec<u8>>, nonce: Arc<Vec<u8>>, counter: u64, blocks_index: &Vec<[u64; 2]>) {
		crossbeam::scope(|scope|{
        	for i in 0..blocks_index.len() as usize {
            	let tx = self.parallel_interface.get_tx(i as u8);
            	let start = blocks_index[i][0] as usize;
            	let end = blocks_index[i][1] as usize;
            	
            	let message = Arc::clone(&message);
            	let nonce = Arc::clone(&nonce);
            	let key = Arc::clone(&key);
                scope.spawn(move ||{
                    let c = counter + start as u64;
                    let ciphertext = jacopone_encrypt_ctr(Arc::new(message[start * 64 .. end * 64].to_vec()), key, nonce, c);
                    tx.send(ciphertext).unwrap();
                });
            	      
        	}
    	});
	} 

}