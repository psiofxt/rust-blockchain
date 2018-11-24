use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use proof::proof_of_work;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    index: usize,
    timestamp: Duration,
    transactions: Vec<Transaction>,
    proof: String,
    previous_hash: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: f32
}

#[derive(Debug)]
struct Node {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn new_block(&mut self) -> Result<(), ()>{
        // define the block and append it to the chain
        let clone_chain = self.clone();
        let last_block = clone_chain.last_block();
        let previous_hash: String = match last_block {
            Some(block) => {
                clone_chain.hash(block)
            },
            None => {
                String::from("NULL")
            }
        };
        let proof: String = proof_of_work(&previous_hash);
        self.new_transaction(
            String::from("0"),
            String::from("0"),
            1.00000000
        );
        let block: Block = Block {
            index: self.chain.len() + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            transactions: self.current_transactions.clone(),
            proof: proof,
            previous_hash: previous_hash,
        };
        self.current_transactions = Vec::<Transaction>::new();
        self.chain.push(block);
        Ok(())
    }

    fn new_transaction(&mut self, sender: String, recipient: String, amount: f32) {
        let transaction = Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount,
        };
        &mut self.current_transactions.push(transaction);
    }

    pub fn last_block(&self) -> Option<&Block> {
        let index: usize = match self.chain.len() {
            0 => 0,
            _ => self.chain.len() - 1
        };
        info!("{:?}", self.chain.get(index));
        self.chain.get(index)
    }

    pub fn hash(&self, block: &Block) -> String {
        let blockstring = json!(block).to_string();
        let mut sha = Sha256::new();
        sha.input_str(&blockstring);
        sha.result_str()
    }
}
