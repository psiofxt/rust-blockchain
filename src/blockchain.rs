use std::collections::HashSet;

struct Block {
    index: i32,
    timestamp: String,
    transactions: String,
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

struct Blockchain {
    chain: Vec<HashSet<Block>>,
    current_transactions: Vec<HashSet<Transaction>>,
    nodes: Vec<HashSet<Node>>,
}


impl Blockchain {
    fn new_block(proof: String, previous_hash: String) {
        // define the block and append it to the chain
    }
}
