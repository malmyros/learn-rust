fn main() {
    
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        even_or_odd(*number)
    }
    
}

fn even_or_odd(i: i32) -> () {
    let is_even = i % 2 == 0;
    if is_even {
        println!("{} is even", i);
    } else {
        println!("{} is odd", i);
    }
}
