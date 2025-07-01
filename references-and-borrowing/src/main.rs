fn main() {
    /*
    Rust allows unlimited amount of immutable references and borrowers
    as there is no risk in reading the value, when we can't mutate it.
     */
    let car: String = String::from("Red");
    let ref1: &String = &car;
    let ref2: &String = &car;
    println!("Ref1: {} and Re2f: {} and Inline Ref: {}", ref1, ref2, &car);

    /*
    A lifetime is a construct the compiler (or more specifically, its borrow checker) 
    uses to ensure all borrows are valid. Specifically, a variable's lifetime begins 
    when it is created and ends when it is destroyed. While lifetimes and scopes are 
    often referred to together, they are not the same.
     */
    let mut plane: String = String::from("Blue");
    let ref3: &mut String = &mut plane;
    ref3.push_str(" and Green");
    let ref4: &String = &ref3;
    println!("Ref3: {}", ref3);
    println!("Ref4: {}", ref4);
    println!("Plane: {}", plane);
}
