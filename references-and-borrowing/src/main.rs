fn main() {

    /*
    Rust allows unlimited amount of immutable references and borrowers
    as there is no risk in reading the value, when we can't mutate it.
     */
    let car: String = String::from("Red");
    let ref1: &String = &car;
    let ref2: &String = &car;
    println!("Ref1: {} and Re2f: {} and Inline Ref: {}", ref1, ref2, &car);
}
