fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        even_or_odd(*number)
    }

    let condition = 20 > 10;
    let result = match condition {
        true => 20,
        false => 10,
    };
    println!("The result is {}", result);

    let season: &str = "autumn";
    match season {
        "summer" => println!("School's out"),
        "winter" => println!("Brr, so cold"),
        _ => println!("Lots of rain"),
    }
}

fn even_or_odd(value: i32) -> () {
    let is_even = value % 2 == 0;
    match is_even {
        true => println!("{} is Even!", value),
        false => println!("{} is Odd!", value),
    }
}
