fn main() {
    open_store("Los Angeles");
    bake_pizza(32, "anchovies");
    swimming_in_profit();
    println!("{}", square(2))
}

fn open_store(neighborhood: &str) {
    println!("Opening a store, in {neighborhood}");    
}

fn bake_pizza(number: i32, toppings: &str) {
    println!("Baking {number} {toppings} pizza!");
}

fn swimming_in_profit() {
    println!("So much $$$, swimming in profit")
}

fn square(number: i32) -> i32 {
    number * number
}