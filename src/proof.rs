use crypto::digest::Digest;
use crypto::sha2::Sha256;


pub fn proof_of_work(last_proof: &String) -> String {
    let mut proof: i32 = 0;
    while !valid_proof(&last_proof, &proof) {
        proof += 1;
    };
    proof.to_string()
}

pub fn valid_proof(last_proof: &String, current_proof: &i32) -> bool {
    let guess: String = format!("{}{}", last_proof, current_proof);
    let mut sha = Sha256::new();
    sha.input_str(&guess);
    let result: String = sha.result_str();
    match &result[0..4] {
        "0000" => {
            println!("{:?}", result);
            return true
        },
        _ => {
            return false
        }
    }
}
