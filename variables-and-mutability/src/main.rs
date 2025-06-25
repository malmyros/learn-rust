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
}
