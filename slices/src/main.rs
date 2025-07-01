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
}
