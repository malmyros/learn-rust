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

    // Float types are always signed
    let pie: f64 = std::f64::consts::PI;
    println!("Pie value is: {}f64", pie);
    println!("Pie value is: {}", pie.floor());
    println!("Pie value is: {}", pie.ceil());
    println!("Pie value is: {}", pie.round());

    // Alternative syntaxes for formatting floats
    println!("Pie value is: {:.5}", pie);
    println!("Pie value is: {pie:.5}");

    // Example of casting type using the as keyword
    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as i8;

    let addition = 5 + 4;
    let subtraction = 5 - 4;
    let multiplication = 5 * 4;

    /*
    Rust performs floor division when dividing an
    integer with another integer, which rounds down
    the result to the nearest integer less than or equal
    to the division result

    5 / 3 = 1.6 -> rounding down to 1
    */
    let division: i32 = 5 / 3;
    println!("Division is: {}", division);

    /*
    On the other hand Rust performs division
    when dividing with float numbers retaining
    the decimal places
     */
    let decimal_division: f64 = 5.0 / 3.0;
    println!("Decimal division is: {}", decimal_division);

    let remainder = 7 % 1;
    println!("Remainder is: {}", remainder);

    let mut year = 2025;
    year += 1;
    println!("year: {}", year);

    year -= 5;
    println!("year: {}", year);

    year *= 10;
    println!("year: {}", year);

    year /= 10;
    println!("year: {}", year);
}
