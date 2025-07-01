fn main() {

    let action_hero: String = String::from("Arnold Schwarzenegger");
    assert_eq!(action_hero.len(), 21);
    
    let first_name: &str = &action_hero[0..6];
    let last_name: &str = &action_hero[6..21];
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);
}
