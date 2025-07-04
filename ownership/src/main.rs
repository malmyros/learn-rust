#![allow(unused_variables)]
fn main() {
    /*
    In rust variables implement the copy trait
    which means in the following example we are
    going to have the value of 2025 twice in the stack
    */
    let time = 2025;

    /*
    This is a full copy of the value of time in the year variable
    Year is the owner and is responsible for cleaning up the value
    when exiting the scope of the main function
    */
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

    /*
    The Stack contains:
        Reference -> The address to the heap location where the text is stored,
        Length of the string, which is the current number of bytes that the text occupies: 7,
        Capacity the amount of bytes available in the heap location: 10
    The Heap contains: Michail
     */
    let mut name = String::from("Michail");
    println!("{}", name);

    name.push_str(" Almyros");
    println!("{}", name);

    /*
    A heap allocated string doesn't implement the copy trait
    and therefore Rust will not make a copy of this heap data
    when person is reassigned to genius, that would create a
    duplicate of heaps text data, which will probably occupy
    more memory that something on the stack.

    Data in the heap can grow, where data in the stack is fixed

    Instead, here we will have 2 references in the stack but
    the ownership is moved from person to genius, which means
    person goes out of scope, and genius will be responsible
    for deallocating the memory.

    If we try to do println!("{}", person);
    then Rust will give the following error:
    Value used after being moved [E0382]
     */
    let person = String::from("Michail");
    let genius = person;

    /*
    Rust will call the drop function when a variable gets out of
    scope, but we can also call it ourselves. After calling the drop
    function if we try to use the variable we will get the error:
    Value used after being moved [E0382]

    We also can't transfer the ownership to another variable
    because the ownership was removed from the variable.
    So doing let someone = genius will cause an error as well
    */
    drop(genius);

    /*
    Because we use clone now there is no transfer of ownership,
    and there are two district pieces of data in the heap
     */
    let new_person = String::from("Michail");
    let new_genius = new_person.clone();

    /*
    The & is the borrow operator because it borrows another value
    meaning it creates a reference.

    Example: &i32 can be read as a reference to i32

    We are no longer cloning, but we are copying the reference to the value
     */
    let my_stack_value: i32 = 2;
    let my_integer_reference: &i32 = &my_stack_value;
    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;

    /*
    The * is the deference operator which means to access the
    data that the reference points to.

    Get the address -> Go to the address -> Get the value of the address
    */
    println!("{}", *my_integer_reference);
    println!("{}", *my_heap_reference);

    /*
    The * operator can only be used with a reference if we try to use
    it with a regular value Rust will give the error:

    Example: println!("{}", *my_heap_value);
    The size for values of type `str` cannot be known at compilation time [E0277]

    Finally, Rust's display trait automatically dereferences
    so we don't need to explicitly, use the operator
     */

    let burger = String::from("Burger");
    let meal = add_fries(burger);
    println!("{}", meal);

    let cake = bake_cake();
    println!("I now have a {} cake", cake);

    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    current_meal = add_sugar(current_meal);
    
    /*
    We need to use the correct type &String
    which is an immutable reference of String
    that uses the & borrow operator.
     */
    show_my_meal(&current_meal);
}

// meal: String 
// mut meal: String

// It's no longer a string but a reference to a string "An address"
// meal: &String

// A mutable reference to a String
// meal: &mut String

fn add_fries(mut meal: String) -> String {
    meal.push_str(" and Fries");
    meal
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str(", Add sugar");
    meal
}

fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn bake_cake() -> String {
    String::from("Chocolate Mousse")
}

/*
We are using &String to declare an immutable reference to
the current_meal.  
 */
fn show_my_meal(current_meal: &String) {
    println!("Meal steps: {}", current_meal);
}