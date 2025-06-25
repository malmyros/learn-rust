#![allow(unused_variables)]
fn main() {
    /*
    Rust automatically infers the type, but we can also assign
    it if we want to be specific
    */

    // Signed Integers can be negative and positive numbers
    let eight_bit: i8 = -112;
    let sixteen_bit: i16 = -32_767;

    // Unsigned Integers can be zero or positive numbers
    let eight_bit_unsinged: u8 = 112;
    let sixteen_bit_unsinged: u16 = 32_768;

    // Alternative syntax (Not commonly used)
    let some_variable = 8i16;

    // Rust allows us to use underscores to improve visibility
    let amount = 1_000_000;

    let days: usize = 55;
    let years: isize = -15_000;

    // Defining a string, which is a collection of characters
    let file_path: &str = "/learn-rust/data_types/src/main.rs";
    println!("Path: {}", file_path);

    // Example of using the r to define the string as raw string
    let raw_string: &str = r"C:/My Documents/new/documents";
    println!("Raw string: {}", raw_string);

    let empty_space: &str = "           my content     ";
    println!("{}", empty_space.trim());

    let n: i32 = 2;
    println!("{}", n.pow(2));   
}
