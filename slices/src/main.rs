fn main() {
    // Immutable String
    let action_hero: String = String::from("Arnold Schwarzenegger");
    println!("Action Hero: {}", action_hero);

    // Immutable reference to a portion of the string
    let action_hero: &str = "Arnold Schwarzenegger";
    assert_eq!(action_hero.len(), 21);

    let first_name: &str = &action_hero[0..6];
    let last_name: &str = &action_hero[6..21];
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);

    /*
    Because we are using &str the value of action here will be
    embedded in the final executable and therefore when we return
    the slice we won't create a dangling reference as the value
    will be referenced from the memory
     */
    let some_part: &str = {
        let action_hero: &str = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };
    println!("Some Part: {}", some_part);

    /*
    The length of a string slice refers to a count
    of its bytes, it's characters
     */
    let food: &str = "Pizza";
    println!("Length: {}", food.len());

    let pizza_slice: &str = &food[0..3];
    println!("Length: {}", pizza_slice.len());

    /*
    The length here is now 4 as it occupies 4 bytes
    in memory and not 1 as in length of characters
    */
    let crab: &str = "🦀";
    println!("Length: {}", crab.len());

    /*
    Syntactic Shortcuts
     */
    let actor_first_name: &str = &action_hero[..6];
    let actor_last_name: &str = &action_hero[7..];
    println!("Actor: {}", actor_first_name);
    println!("Actor: {}", actor_last_name);

    let actor_full_name: &str = &action_hero[..];
    println!("Actor: {}", actor_full_name);
    do_hero_stuff(action_hero);
    
    let languages: [&str; 3] = ["Rust", "Go", "Java"];
    let language_slices: &[&str] = &languages[..2];
    println!("Language slices: {:?}", language_slices);
    
    let values: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let regular_reference: &[i32; 6] = &values;
    let slice_of_three = &values[..3];
    print_length(regular_reference);
    print_length(slice_of_three);

    /*
    Rust does not permit mutable slices of Strings,
    if we are borrowing a string slice we can only do it immutably.

    However, Rust does permit mutable slices of arrays
     */
    let mut some_values: [i32; 4] = [1, 2, 3, 4];

    // An immutable slice
    let my_slice = &some_values[2..4];
    println!("my_slice: {:?}", my_slice);

    // A mutable slice
    let mutable_slice = &mut some_values[2..4];
    mutable_slice[0] = 10;
    println!("mutable_slice: {:?}", mutable_slice);
    println!("some_values: {:?}", some_values);
}
fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}

fn print_length(reference: &[i32]) {
    println!("The length of reference is {}", reference.len());
}