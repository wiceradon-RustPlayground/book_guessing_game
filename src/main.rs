use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn cmp(val1: u32, val2: u32) -> Ordering {
    if val1 < val2 { Ordering::Less }
    else if val1 > val2 { Ordering::Greater }
    else { Ordering::Equal }
}

fn main() {
    let random = (rand::random::<u32>() % 10) + 1;
    println!("Hello, it's a guessing game!");
    println!("I'll draw a number between 1-10 and you need to guess which one.");

    loop {
        println!("Please enter your number");
        let input = old_io::stdin().read_line().ok().expect("You need to pass nummber");

        // It seems, that we need to remove newline before parsing
        let parsed_input: Result<u32, _> = input.trim().parse();

        let num = match parsed_input {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number!");
                return;
            }
        };

        match cmp(num, random) {
            Ordering::Less => println!("Your number is too low!"),
            Ordering::Greater => println!("Your number is too big!"),
            Ordering::Equal => {
                println!("You have won!");
                break;
            },
        }
    }
}
