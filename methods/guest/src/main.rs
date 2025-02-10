// use risc0_zkvm::guest::env;

// fn main() {
//     // TODO: Implement your guest code here

//     // read the input
//     let input: u32 = env::read();

//     // TODO: do something with the input

//     // write public output to the journal
//     env::commit(&input);
// }


// #![no_main]


use risc0_zkvm::guest::{env, sha};
risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    // Read input
    let sentence: String = env::read();

    let mut is_okay = false;
    if sentence.contains("hello world") {
        is_okay = true;
    } else {
        is_okay = false;
    }

    if !is_okay {
        panic!();
    }
    let digest: Digest = sha::digest_u8_bytes(sentence.as_bytes());
    env::commit(digest);
}
