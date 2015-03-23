use std::old_io;
use std::rand;

fn main() {
    let random = (rand::random::<u32>() % 10) + 1;
    println!("{}", random);
    println!("Hello, it's a guessing game!");
    println!("I'll draw a number between 1-10 and you need to guess which one.");
    println!("Only one shot!");

    let input = old_io::stdin().read_line().ok().expect("You need to pass nummber");

    // It seems, that we need to remove newline before parsing (bug?)
    let parsed_input: Result<u32, _> = input.trim_right_matches("\n").parse();

    let num = match parsed_input {
        Ok(num) => num,
        Err(_) => {
            println!("Please input number!");
            return;
        }
    };

    if random == num {
        println!("You won!");
    } else {
        println!("Looser!");
    }
}
