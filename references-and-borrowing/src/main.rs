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

    /*
    Immutable references in Rust implement the copy trait,
    so Rust will create a full copy of reference a and assign it to b.
    So a remains valid and remains the owner of its own immutable reference
    Where b represents a copy a separate entry on the stack that is the owner of
     */
    let coffee: String = String::from("Mocha");
    let a: &String = &coffee;
    let b: &String = &a;
    println!("{}, {}", a, b);

    /*
    On the other hand mutable references in Rust do not implement the copy trait.
     */
    let mut new_coffee = String::from("Mocha");
    let c: &mut String = &mut new_coffee;

    /*
    Because mutable references don't implement the copy trait
    if we try to print both variables we will get the following error
    cannot borrow `new_coffee` as immutable because it is also borrowed as mutable [E0502]
    println!("{}, {}", new_coffee, c);
    */
    println!("{}", c);
    println!("{}", new_coffee);
}
