#![allow(unused_variables)]
fn main() {
    /*
    In rust variables implement the copy trait
    which means in the following example we are
    going to have the value of 2025 twice in the stack
    */
    let time = 2025;
    let year = time;

    /*
    The &str is embedded directly to the compiler executable,
    and it's not stored either in the stack or the heap, this
    happens because the value is already know at the compile time
     */
    let food: &str = "past";

    /*
    String is a different type in rust which is used
    to dynamically build strings as the value of the string
    is not known at the compile time, it acts as String builder
    this type would be stored in the heap
     */
    let text = String::new();
    let candy = String::from("KitKat");
    
}
