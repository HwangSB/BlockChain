use chrono::Utc;

use sha2::{Digest, Sha256};

pub type BlockHash = Vec<u8>;
pub type BlockData = Vec<u8>;

#[derive(Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: String,
    pub prev_hash: BlockHash,
    pub curr_hash: BlockHash,
    pub nonce: u32,
    pub data: BlockData
}

impl Block {
    pub fn new() -> Self {
        Block {
            index: 0,
            timestamp: Utc::now().to_rfc3339(),
            prev_hash: vec![0; 32],
            curr_hash: vec![0; 32],
            nonce: 0,
            data: vec![0; 32]
        }
    }

    pub fn cacluate_hash(block: &Block) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(block.index.to_be_bytes());
        hasher.update(block.timestamp.as_bytes());
        hasher.update(&block.prev_hash);
        hasher.update(block.nonce.to_be_bytes());
        hasher.update(&block.data);
        hasher.finalize().to_vec()
    }
}

pub struct ProofOfWork;

impl ProofOfWork {
    pub fn run(block: &mut Block, target: BlockHash) {
        let mut nonce = 0;
        
        while nonce < u32::MAX {
            block.nonce = nonce;
            block.curr_hash = Block::cacluate_hash(block);
            
            if Self::validate(block, &target) == -1 { break; }
            else { nonce += 1; }
        }
    }

    pub fn validate(block: &Block, target: &BlockHash) -> i8 {
        for (&x, &y) in block.curr_hash.iter().zip(target.iter()) {
            if x < y { return -1 }
            if x == y { continue }
            if x > y { return 1 }
        }
        0
    }
}