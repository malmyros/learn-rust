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

    println!("mocha name: {}", mocha.name);
    println!("mocha price: £{:.2}", mocha.price);
    println!("mocha is_hot: {}", mocha.is_hot);

    /*
    Doing this will move ownership of the name field
    from the mocha object to the my_favourite_coffee

    let my_favourite_coffee = mocha.name;

    Before of this the compiler won't allow us to
    do something like this:

    println!("{}", mocha.name);
     */

    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 5.64,
        is_hot: false,
    };

    beverage.name = String::from("Latte Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = false;
    
    print_coffee(&mocha);
    print_coffee(&beverage);
}

struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn print_coffee(coffee: &Coffee) {
    println!(
        "Coffee: {}, costs {}, and {}",
        coffee.name, coffee.price, if coffee.is_hot {"it's hot"} else {"it's not hot"}
    );
}
