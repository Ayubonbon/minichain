use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Block {
    username: u16,
    index: u64,
    timestamp: u64,
    previous_hash: String,
    hash: String,
    data: String,
    nonce: u64,
}

impl Block {
    fn new(username: u16, index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut block = Block {
            username,
            index,
            timestamp,
            previous_hash,
            hash: String::new(),
            data,
            nonce: 0,
        };

        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}{}",
            self.username, self.index, self.timestamp, self.previous_hash, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block #{}:\n  User ID: {}\n  Timestamp: {}\n  Data: {}\n  Prev Hash: {}\n  Hash: {}\n  Nonce: {}\n",
            self.index, self.username, self.timestamp, self.data, self.previous_hash, self.hash, self.nonce
        )
    }
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block::new(0, 0, String::from("0"), String::from("Genesis Block"));
        genesis_block.mine_block(difficulty);

        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    fn add_block(&mut self, username: u16, data: String) {
        let prev_block = self.chain.last().unwrap();
        let mut new_block = Block::new(username, prev_block.index + 1, prev_block.hash.clone(), data);
        
        println!("Mining block {}...", new_block.index);
        new_block.mine_block(self.difficulty);
        println!("Block mined: {}\n", new_block.hash);
        
        self.chain.push(new_block);
    }

    fn is_chain_valid(&self) -> bool {
        let target = "0".repeat(self.difficulty);

        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Verify current block's hash integrity
            if current.hash != current.calculate_hash() {
                return false;
            }

            // Verify chain linkage
            if current.previous_hash != previous.hash {
                return false;
            }

            // Verify proof of work
            if !current.hash.starts_with(&target) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let difficulty = 2;
    let mut my_blockchain = Blockchain::new(difficulty);

    println!("--- Adding Blocks ---");
    my_blockchain.add_block(101, String::from("Alice sent 10 coins to Bob"));
    my_blockchain.add_block(102, String::from("Bob sent 5 coins to Charlie"));

    println!("--- Blockchain State ---");
    for block in &my_blockchain.chain {
        println!("{}", block);
    }

    println!("Is blockchain valid? {}", my_blockchain.is_chain_valid());
}
