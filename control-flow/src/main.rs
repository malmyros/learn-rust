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

    let number: i32 = 8;
    match number {
        2 | 4 | 6 | 8 => println!("The number {} is even", number),
        1 | 3 | 5 | 9 => println!("The number {} is odd", number),
        _ => println!("The number {} is an invalid case", number),
    }

    match number {
        value if value % 2 == 0 => println!("The number {} is even", number),
        _ => unreachable!(),
    }

    let mut seconds = 10;
    loop {
        if seconds == 0 {
            println!("Blastoof! {}", seconds);
            break;
        }

        println!("{} seconds to blastoff...", seconds);
        seconds -= 1;
    }

    seconds = 10;
    while seconds > 0 {
        println!("{} seconds to blastoff...", seconds);
        seconds -= 1;
    }

    countdown(5);
}

fn even_or_odd(value: i32) -> () {
    let is_even = value % 2 == 0;
    match is_even {
        true => println!("{} is Even!", value),
        false => println!("{} is Odd!", value),
    }
}

fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
        return;
    } else {
        println!("seconds {} to blastoff...", seconds);
        countdown(seconds - 1)
    }
}
