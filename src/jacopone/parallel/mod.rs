use crypto::sha3::Sha3;
use crypto::digest::Digest;
use super::utils::*;
pub mod parallelinterface;

/*
pub struct CipherData{
	message: &[u8],
	key: &[u8],
	nonce: &[u8],
	counter: &[u8],
}*/

pub struct Parallel {
	thread_count: u8,
	encrypt_fn: fn(&[u8], &[u8], &[u8], u64) -> Vec<u8>,
	parallel_interface: parallelinterface::ParallelInterface,
}

impl Parallel {

	pub fn new (n: u8, encrypt_fn: fn(&[u8], &[u8], &[u8], u64) -> Vec<u8>) -> Parallel{
		let interface = parallelinterface::ParallelInterface::new(n);
		Parallel {thread_count: n, encrypt_fn: encrypt_fn, parallel_interface: interface}
	}

	pub fn encrypt(&self, message: &[u8], key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
		/*let message = data.message;
		let key = data.key;
		let nonce = data.nonce;
		let counter = data.counter;*/

		let blocks_index = get_thread_blocks(message.len(), self.thread_count);
		let mut active_threads = 0;


    	//spawnw thread_count threads
    	crossbeam::scope(|scope|{
        	for i in 0..self.thread_count as usize {
            	let tx = self.parallel_interface.get_tx(i as u8);
            	let start = blocks_index[i][0] as usize;
            	let end = blocks_index[i][1] as usize;
            	if end - start > 0 {
            		active_threads = active_threads + 1;
                	scope.spawn(move ||{
                    	let c = counter + start as u64;
                    	let ciphertext = jacopone_encrypt_ctr(&message[start * 64 .. end * 64], key, nonce, c);
                    	tx.send(ciphertext).unwrap();
                	});
            	}      
        	}
    	});

    	self.parallel_interface.concat(active_threads)
	} 

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

pub fn block_encrypt(message: &[u8], key: &[u8]) -> Vec<u8> {
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
