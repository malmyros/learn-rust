/*
A struct (structure) is a container for
related pieces of data.

Types of Structs are:
    - Named Field Structs
    - Tuple-Like Structs
    - Unit-Like Structs

An instance is the concrete value made from a type
 */

fn main() {
    let mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 5.64,
        is_hot: true,
    };
    
    
}

struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}
