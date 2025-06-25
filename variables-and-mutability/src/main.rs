// Example of using compiler directives globally
// #![allow(unused_variables)]
const TAX_RATE: f64 = 7.25;

type Meters = i32;

fn main() {
    // Allocating values to immutable variables
    let apples: i32 = 5;
    let oranges: i32 = 14 + 6;
    let fruits: i32 = apples + oranges;

    // String interpolation
    println!("My garden has {fruits} fruits");
    println!("Apples are {0} and oranges are {1}", apples, oranges);

    /*
    Example of an unused variable prefixed with _
    to ignore compiler warnings (I wouldn't use that)
    */
    let _test: &str = "Some text to ignore warnings";

    // Example of using mut to define a variable as mutable
    let mut gym_reps: i32 = 10;
    println!("My Gym-Reps are {gym_reps}");

    gym_reps += 10;
    println!("My new Gym-Reps are {gym_reps}");

    /*
    If the variable wasn't mutable we could use explain
    to find out more information
    Ex. rustc --explain E0384
    */

    /*
    Example of variable shadowing for
    resuing the same variable name
    */
    let _grams_of_protein: &str = "100.345";
    let grams_of_protein: f64 = 100.345;
    println!("My protein shake has {grams_of_protein} grams of protein");

    /*
    Example of defining an inner scope to show that
    a variable with the same name won't be a shadow variable
     */
    let cookie_price: f64 = 2.00;
    {
        let cookie_price: f64 = 2.45;

        // Value would be: My cookie price is 2.45
        println!("My cookie price is {:.2}", cookie_price);
    }

    // Value would be: My cookie price is 2
    println!("My cookie price is {:.2}", cookie_price);

    // Printing a constant and formatting it to 2 decimal places
    println!("Current tax rate is {:.2}", TAX_RATE);

    // Example of using type alias
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;
    println!(
        "Mile race length is {0} meters and a two mile race length is {1} meters",
        mile_race_length, two_mile_race_length
    );

    // Example of using compiler directives
    #[allow(unused_variables)]
    let some_new_miles: Meters = 2000;
}
