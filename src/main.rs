mod models;
use chrono::{DateTime, Utc};
use models::{Block, Blockchain};
use std::time::{SystemTime, UNIX_EPOCH};
// mining difficulty
const DIFFICULTY: usize = 4;

fn main() {
    println!(" Welcome to the blockchain sim!");

    println!("Enter a name for your miner");

    let mut miner_name: String = String::new();

    std::io::stdin()
        .read_line(&mut miner_name)
        .expect("Failed to read input");

    miner_name = miner_name.trim().to_string();

    let trader_names: Vec<&str> = vec!["Sarah", "Tom", "Clark", "Joe", "Saul", "Paul", "Mark"];

    let mut bcoin: Blockchain = Blockchain::new();

    println!("\n Lets start mining\n");

    let mut sender: String = miner_name.clone();

    for i in 0..trader_names.len() {
        println!("Mining Block {}....", i + 1);
        let recipient: String = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            miner_name.clone()
        };
        let transaction: String = format!("{} sent to {}", sender, recipient);

        let new_block: Block = Block::new((i + 1) as u32, String::new(), transaction.clone());

        bcoin.add_block(new_block);

        println!("Transaction: {}", transaction);

        sender = recipient;

        println!();
    }

    let total_block: usize = bcoin.get_total_block();

    print!("Total blocks added to blockchain: {}", total_block);

    let bcoin_per_block: usize = 137;

    let bcoin_traded: usize = total_block * bcoin_per_block;

    println!("Total bcoin traded: {} bcoin", bcoin_traded);

    let end_time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time")
        .as_secs();

    let end_date: Option<DateTime<Utc>> = chrono::DateTime::from_timestamp(end_time as i64, 0);

    println!("Sim ended at: {:?}", end_date);

    println!("Mining complete");
}
