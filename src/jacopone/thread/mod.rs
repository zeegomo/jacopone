pub use super::utils::*;
use std::sync::Arc;
pub mod parallelinterface;



pub struct FinalThread {

}

impl FinalThread {
	pub fn finalize_encryption(data: CipherData) -> Vec<u8> {
		let mut c = data.counter + (data.message.len()/64) as u64;
        let block_counter = get_block_counter(&(data.nonce), & mut c);
        xor(&(data.message[data.message.len()/64 * 64..]), &block_encrypt(&block_counter, &(data.key)))
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

	pub fn encrypt(&self, data: CipherData) -> Vec<u8> {
		let blocks_index = get_thread_blocks(data.message.len(), self.thread_count);
    	self.spawn_threads(data, &blocks_index);
    	self.parallel_interface.concat(blocks_index.len() as u8)
	}

	fn spawn_threads(&self, data: CipherData, blocks_index: &Vec<[u64; 2]>) {
		crossbeam::scope(|scope|{
        	for i in 0..blocks_index.len() as usize {
            	let tx = self.parallel_interface.get_tx(i as u8);
            	let start = blocks_index[i][0] as usize;
            	let end = blocks_index[i][1] as usize;
            	
            	/*
            	let message = Arc::clone(&(data.message));
            	let nonce = Arc::clone(&(data.nonce));
            	let key = Arc::clone(&(data.key));
            	let counter = data.counter;*/
            	let data = CipherData::clone(&data);

                scope.spawn(move ||{
                    let c = data.counter + start as u64;
                    let ciphertext = jacopone_encrypt_ctr(Arc::new(data.message[start * 64 .. end * 64].to_vec()), data.key, data.nonce, c);
                    tx.send(ciphertext).unwrap();
                });
            	      
        	}
    	});
	} 

}