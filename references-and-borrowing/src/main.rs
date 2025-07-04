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

    /*
    A dangling reference is a pointer
    to a memory address that has been deallocated

    Dangle means to hang loosely or unreliably

    So a dangling reference is an unreliable reference to an address
    let city: String = create_city();
     */

    /*
    registrations is the owner of the array which is the owner
    of these 3 elements and remains the owner of the 3 booleans
    even when we extract the value from the array
    */
    let registrations: [bool; 3] = [true, false, true];

    /*
    The bool type implements the copy trait and therefore when we
    extract the first element Rust creates a full copy of the boolean
    and assigns it to the first_element variable
    */
    let first_element: bool = registrations[0];
    println!("First element is {}", first_element);

    /*
    The String type does not implement the copy trait which means
    that ownership would move from languages to the first_language variable.
    
    Because of this we need to use the clone method on the String type that
    does not move the ownership, rather it creates a full copy.
    */
    let languages: [String; 2] = [String::from("Rust"), String::from("Go")];
    let first_language = languages[0].clone();
    println!("First Language: {}", first_language);
    
    /*
    Instead of coping the value and using additional memory we can borrow a reference,
    which will also prevent the movement of ownership from the languages array 
     */
    let first_language_reference: &String = &languages[0];
    println!("First Language Reference: {}", first_language_reference);
    
    /*
    Example of using borrow reference with tuples
     */
    let alternative_languages = (String::from("Rust"), String::from("Go"));
    let first_alternative_language: String = alternative_languages.0.clone();
    let first_alternative_language_reference: &String = &alternative_languages.0;
    println!("Alternative Language: {}", first_alternative_language);
    println!("Alternative Language Reference: {}", first_alternative_language_reference);
}

/*
Example of a function that would return a dangling reference
fn create_city() -> &String {
let city: String = String::from("Athens");
&city
}

The Rust Compiler won't allow us to do this because at end of the
function the city variable gets out of scope and therefore there
won't be any value to reference anymore.

Instead, the Rust Compiler suggests us to return the variable as is
fn create_city() -> String {
    let city: String = String::from("Athens");
    city
}

or

fn create_city() -> String {
    String::from("Athens")
}
*/
