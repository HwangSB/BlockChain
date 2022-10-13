pub mod block;
pub use block::{Block, ProofOfWork};

pub mod permanence;
pub use permanence::Permanence;

pub struct BlockChain<T> {
    pub difficult: usize,
    chain: Vec<Block>,
    permanence: T,
}

impl<T> BlockChain<T>
where T: Permanence {
    pub fn new(permanence: T) -> Self {
        BlockChain {
            difficult: 28,
            chain: Vec::new(),
            permanence
        }
    }

    pub fn add(&mut self, mut block: Block) -> &Block {
        block.index = self.chain.len() + 1;

        let prev_block = self.permanence.read(0);
        if let Ok(prev_block) = prev_block {
            block.prev_hash = prev_block.curr_hash.clone();
        }

        let target = Self::target_from(self.difficult);
        ProofOfWork::run(&mut block, target);

        self.permanence.write(&block);

        self.chain.push(block);

        self.chain.last().unwrap()
    }
    
    pub fn _validate(&self) -> bool {
        let target = Self::target_from(self.difficult);
        
        let mut valid = true;
        for block in &self.chain {
            if ProofOfWork::validate(block, &target) != -1 {
                valid = false;
            }
        }

        valid
    }

    fn target_from(difficult: usize) -> Vec<u8> {
        let mut target = vec![0u8; 32];
        let mut digit = 0;
        let target_digit = (256 - difficult * 8) / 16 - 1;
        for x in &mut target {
            if digit == target_digit { *x = 0x01; }
            else { *x = 0; }
            digit += 1;
        }

        target
    }
}

pub struct BlockChainIntoIterator<T> {
    offset: usize,
    block_chain: BlockChain<T>
}

impl<T> IntoIterator for BlockChain<T>
where T: Permanence {
    type Item = Block;
    type IntoIter = BlockChainIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        BlockChainIntoIterator {
            offset: 0,
            block_chain: self,
        }
    }
}

impl<T> Iterator for BlockChainIntoIterator<T>
where T: Permanence {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.block_chain.permanence.read(self.offset);
        self.offset += 1;

        if let Ok(block) = result {
            return Some(block);
        }

        return None;
    }
}