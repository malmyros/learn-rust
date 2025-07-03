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

    let coffee: Coffee = make_coffee(String::from("Brown sugar oat shaken espresso"), 5.33, false);
    print_coffee(&coffee);

    let new_coffee = Coffee {
        name: String::from("New coffee"),
        ..coffee
    };
    print_coffee(&new_coffee);

    /*
    Strings in Rust don't implement the copy trait,
    and therefore we have to clone the value otherwise
    we will have issue with the ownership of the name
    */
    let another_coffee = Coffee {
        name: new_coffee.name.clone(),
        ..coffee
    };
    print_coffee(&another_coffee);

    // Printing using the standard debug trait implementation
    println!("{:#?}", another_coffee);
}

#[derive(Debug)]
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn print_coffee(coffee: &Coffee) {
    println!(
        "Coffee: {}, costs {}, and {}",
        coffee.name,
        coffee.price,
        if coffee.is_hot {
            "it's hot"
        } else {
            "it's not hot"
        }
    );
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}

// Passing an immutable Coffee Struct instance
#[allow(dead_code)]
fn drink_coffee(coffee: Coffee) {
    println!("Drinking my delicious {}", coffee.name);
}

// Passing a mutable Coffee Struct instance
#[allow(dead_code)]
fn drink_coffee_mutable(mut coffee: Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
}

/*
Passing a reference to a coffee, an address in memory
that we can follow to get a coffee from memory

&Coffee is a different type than Coffee

The &Coffee is read only as it's immutable
*/
#[allow(dead_code)]
fn drink_coffee_reference(coffee: &Coffee) {
    println!("Drinking my delicious {}", coffee.name);
}

#[allow(dead_code)]
fn drink_coffee_mutable_reference(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
}