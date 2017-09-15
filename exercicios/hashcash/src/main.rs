extern crate ring;
extern crate rand;
use ring::digest;

// Exercicio 1: Como minerar vários blocos em paralelo?
fn main() {
    let miner = "leoyvens";
    let block : u64 = rand::random();
    let difficulty = 3;
    validate(miner, block, difficulty);
}

fn validate(miner: &str, block : u64, difficulty: usize) -> u64 {
    let value = [miner.as_bytes(), block.to_string().as_bytes()].concat();
    // Exercicio 2: Como minerar um bloco em paralelo?
    // Se uma thread minerar o bloco, todas devem parar!
    for counter in 0.. {
        let mut value = value.clone();
        value.extend(counter.to_string().as_bytes());
        let digest = digest::digest(&digest::SHA256, &value);
        let hash = digest.as_ref();
        if hash[..difficulty].iter().all(|x| *x == 0) {
            println!("Miner: {} Block: {:?} Counter: {}", miner, block, counter);
            return counter
        }
    }
    unreachable!()
}
