use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    sp1_sdk::utils::setup_logger();
    let mut stdin = SP1Stdin::new();
    let test_number = 42;
    stdin.write(&test_number);

    let client = ProverClient::from_env(); // Updated from new()
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, &stdin).run().expect("Proof generation failed"); // Added &stdin
    println!("Proof generated successfully");

    let public_values_bytes = proof.public_values.as_slice();
    println!("Public values (raw bytes): {:?}", public_values_bytes);
    let is_even = proof.public_values.read::<bool>();
    println!("Input number: {}", test_number);
    println!("Is even? {}", is_even);

    match client.verify(&proof, &vk) {
        Ok(()) => println!("Successfully generated and verified proof!"),
        Err(e) => println!("Verification failed: {:?}", e),
    }
}
