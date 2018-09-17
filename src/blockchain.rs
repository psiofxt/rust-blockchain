use std::collections::HashSet;

struct Block {
    index: u32,
    timestamp: String,
    transactions: u32,
    proof: String,
    previous_hash: String
}

struct Transaction {
    sender: String,
    recipient: String,
    amount: f32
}

struct Node {

}

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    nodes: Vec<HashSet<Node>>,
}


impl Blockchain {

    pub fn new_block(&mut self, proof: String, previous_hash: String){
        // define the block and append it to the chain
        let block = Block {
            index: 1,
            timestamp: String::from("1"),
            transactions: 1,
            proof: proof,
            previous_hash: previous_hash,
        };
        // let self.current_transactions = [];
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

    fn first_name(&self) -> usize {
        self.chain.len()
    }
}
