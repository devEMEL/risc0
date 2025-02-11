// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use risc0_hello::multiply;
use risc0_hello_methods::{
    RISC0_ELF, RISC0_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::Receipt;

fn main() {

    // For example:
    // let input: u32 = 15 * u32::pow(2, 27) + 1;
    // let input = String::from("guys hello world, how is it going?");
    // let env = ExecutorEnv::builder()
    //     .write(&input)
    //     .unwrap()
    //     .build()
    //     .unwrap();


    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Pick two numbers
    let (receipt, _) = multiply(17, 23);

    // Verify receipt, panic if it's wrong
    receipt.verify(RISC0_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );




}


pub fn multiply(a: u64, b: u64) -> (Receipt, u64) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, RISC0_ELF).unwrap().receipt;

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the product
    println!("I know the factors of {}, and I can prove it!", c);

    (receipt, c)
}