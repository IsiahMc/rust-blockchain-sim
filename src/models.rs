use chrono::NaiveDateTime;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fmt::format;
use std::iter::repeat;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::DIFFICULTY;
// block struct and impl
pub struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    nonce: u64,
    hash: String,
    data: String,
}

impl Block {
    pub fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time error")
            .as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&mut self) -> String {
        // concat atr into single string
        let data: String = format!(
            "{}{}{}{}{}",
            self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str: String = format!("{:x}", result);
        hash_str
    }
    fn mine_block(&mut self) {
        let mut iterations: i32 = 0;
        loop {
            // set hash to calc hash and incr iter
            self.hash = self.calculate_hash();
            iterations += 1;
            // if hash empty and diff is 0 repeat
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!("--Block mined {}:", self.index);
                break;
            }

            if iterations > 100 {
                print!("--mining in progress");
                thread::sleep(Duration::from_millis(3000));
                println!("--calculated hash {}:", self.hash);
                break;
            }
            self.nonce += 1;
        }
    }
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime: NaiveDateTime =
            chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}
// blockchain struct and impl
pub struct Blockchain {
    chain: Vec<Block>,
}
impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block: Block = Block::new(0, String::new(), String::from("genisis block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    // add block
    pub fn add_block(&mut self, mut new_block: Block) {
        let previous_hash: String = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block();
        self.chain.push(new_block);
    }
    // get total block
    pub fn get_total_block(&self) -> usize {
        self.chain.len()
    }
}
