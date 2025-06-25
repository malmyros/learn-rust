fn main() {
    // Allocating values to immutable variables
    let apples: i32 = 5;
    let oranges: i32 = 14 + 6;
    let fruits = apples + oranges;

    // String interpolation
    println!("My garden has {fruits} fruits");
    println!("Apples are {0} and oranges are {1}", apples, oranges);
}
