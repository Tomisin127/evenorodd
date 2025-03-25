#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let number = sp1_zkvm::io::read::<u32>();
    let is_even = number % 2 == 0;
    sp1_zkvm::io::commit(&is_even);
}
