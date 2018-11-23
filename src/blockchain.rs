use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Debug)]
struct Block {
    index: usize,
    timestamp: Duration,
    transactions: Vec<Transaction>,
    proof: String,
    previous_hash: String
}

#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: f32
}

#[derive(Debug)]
struct Node {

}

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>
}

impl Default for Blockchain {
    fn default() -> Blockchain {
        Blockchain {
            chain: Vec::<Block>::new(),
            current_transactions: Vec::<Transaction>::new()
        }
    }
}


impl Blockchain {
    pub fn new() -> Blockchain{
        Blockchain {
            chain: Default::default(),
            current_transactions: Default::default()
        }
    }

    pub fn new_block(&mut self, proof: String, previous_hash: String){
        // define the block and append it to the chain
        //let current_transactions: Vec<Transaction> = self.current_transactions.clone();
        let block: Block = Block {
            index: self.chain.len() + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            transactions: self.current_transactions.clone(),
            proof: proof,
            previous_hash: previous_hash,
        };
        self.current_transactions = Vec::<Transaction>::new();
        self.chain.push(block);
    }

    fn new_transaction(&mut self, sender: String, recipient: String, amount: f32) {
        let transaction = Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount,
        };
        &mut self.current_transactions.push(transaction);
    }

    fn last_block(&self) -> usize {
        self.chain.len()
    }
}
