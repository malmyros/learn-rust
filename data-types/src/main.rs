#![allow(unused_variables)]
fn main() {
    /*
    Rust automatically infers the type, but we can also assign
    it if we want to be specific
    */

    // Signed Integers can be negative and positive numbers
    let eight_bit: i8 = -112;
    let sixteen_bit: i16 = -32767;

    // Unsigned Integers can be zero or positive numbers
    let eight_bit_unsinged: u8 = 112;
    let sixteen_bit_unsinged: u16 = 32768;
    
    // Alternative syntax (Not commonly used)
    let some_variable = 8i16;
    
    // Rust allows us to use underscores to improve visibility
    let amount = 1_000_000;
}
