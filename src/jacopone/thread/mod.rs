pub use super::utils::*;
pub mod parallelinterface;

/*
pub struct CipherData{
	message: &[u8],
	key: &[u8],
	nonce: &[u8],
	counter: &[u8],
}*/
pub struct FinalThread {

}

impl FinalThread {
	pub fn finalize_encryption(message: &[u8], key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
		let mut c = counter + (message.len()/64) as u64;
        let block_counter = get_block_counter(nonce, & mut c);
        xor(&message[message.len()/64 * 64..], &block_encrypt(&block_counter, key))
	}
}


pub struct ParallelThread {
	thread_count: u8,
	encrypt_fn: fn(&[u8], &[u8], &[u8], u64) -> Vec<u8>,
	parallel_interface: parallelinterface::ParallelInterface,
	active_threads: u8,
}

impl ParallelThread {

	pub fn new (n: u8, encrypt_fn: fn(&[u8], &[u8], &[u8], u64) -> Vec<u8>) -> ParallelThread{
		let interface = parallelinterface::ParallelInterface::new(n);
		ParallelThread {thread_count: n, encrypt_fn: encrypt_fn, parallel_interface: interface, active_threads: 0}
	}

	pub fn encrypt(& mut self, message: &[u8], key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
		/*let message = data.message;
		let key = data.key;
		let nonce = data.nonce;
		let counter = data.counter;*/

		let blocks_index = get_thread_blocks(message.len(), self.thread_count);


    	//spawnw thread_count threads
    	self.spawn_threads(message, key, nonce, counter, &blocks_index);
    	let result = self.parallel_interface.concat(self.active_threads);
    	self.active_threads = 0;
    	result
	}

	fn spawn_threads(&mut self, message: &[u8], key: &[u8], nonce: &[u8], counter: u64, blocks_index: &Vec<[u64; 2]>) {
		crossbeam::scope(|scope|{
        	for i in 0..self.thread_count as usize {
            	let tx = self.parallel_interface.get_tx(i as u8);
            	let start = blocks_index[i][0] as usize;
            	let end = blocks_index[i][1] as usize;
            	if end - start > 0 {
            		self.active_threads = self.active_threads + 1;
                	scope.spawn(move ||{
                    	let c = counter + start as u64;
                    	let ciphertext = jacopone_encrypt_ctr(&message[start * 64 .. end * 64], key, nonce, c);
                    	tx.send(ciphertext).unwrap();
                	});
            	}      
        	}
    	});
	} 

}