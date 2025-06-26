fn main() {
    open_store("Los Angeles");
    bake_pizza(32, "anchovies");
    swimming_in_profit();
    println!("{}", square(2));

    // Returning a unit
    let result: () = ();
    mystery();

    let multiplier: i32 = 3;
    let calculation: i32 = {
        let value: i32 = 5 + 4;
        value * multiplier
    };
    println!("Result: {calculation}");
}

fn open_store(neighborhood: &str) {
    println!("Opening a store, in {neighborhood}");
}

fn bake_pizza(number: i32, toppings: &str) {
    println!("Baking {number} {toppings} pizza!");
}

fn swimming_in_profit() {
    println!("So much $$$, swimming in profit");
}

fn square(number: i32) -> i32 {
    number * number
}

fn mystery() -> () {
    println!("Function that returns a unit");
}
