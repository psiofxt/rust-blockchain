use std::collections::HashSet;

struct Block {
    index: i32,
    timestamp: String,
    transactions: Vec<HashSet<Transaction>>,
    proof: i32,
    previous_hash: String
}

struct Transaction {
    sender: String,
    recipient: String,
    amount: f32
}

struct Node {

}

struct Blockchain {
    chain: Vec<HashSet<Block>>,
    current_transactions: Vec<HashSet<Transaction>>,
    nodes: Vec<HashSet<Node>>,
}


impl Blockchain {

    fn new_block(&self, proof: String, previous_hash: String) {
        // define the block and append it to the chain
        let block = Block {
            index: self.chain.len(),
            timestamp: String::("1"),
            transactions: self.current_transactions,
            proof: proof,
            previous_hash: previous_hash,
        }
        let self.current_transactions = [];
        self.chain.push(block)
        block;
    }

    fn new_transaction(&self, sender: String, recipient: String, amount: f32) {
        let transaction = Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount,
        }
        self.current_transactions.push(transaction)
    }

    fn first_name(&self) -> &u32 {
        &self.chain.len()
    }
}
