extern crate rand;

use rand::Rng;
use std::io::{self, Write};

// struct Die {
//     sides: i8,
//     modifier: i8,
// }

// Project Trello: https://trello.com/b/8QqyFxg6/rust

fn main() {
    loop {
        let input = input();
        match input.trim() {
            "d2+4" => formatted_roll(2,4),
            "d2" => formatted_roll(2,0),
            "d4" => formatted_roll(4,0),
            "d6" => formatted_roll(6,0),
            "d8" => formatted_roll(8,0),
            "d10" => formatted_roll(10,0),
            "d12" => formatted_roll(12,0),
            "d20" | "r" => formatted_roll(20,0),
            "d100" => formatted_roll(100,0),
            "q" => break,
            "?" => help(),
            "" => (),
            _ => println!("! Invalid Command (enter ? for help)"),
        }
    }
}

fn help() {
    println!(" ------------------------------------------------ ");
    println!("|                                                |");
    println!("| r: to roll a d20                               |");
    println!("| dx: to roll where x is the number of sides     |");
    println!("|     2, 4, 6, 8, 10, 12, 20, & 100 are valid    |");
    println!("| q: quit the program                            |");
    println!("| ?: bring up this help menu                     |");
    println!("|                                                |");
    println!(" ------------------------------------------------ ");
}

// Add string parsing for dice modifiers?
fn input() -> std::string::String {
    let mut input_string = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    return input_string;
}

fn formatted_roll(sides: i8, modifier: i8) {
    println!("> You rolled a: {}", roll(sides, modifier));
}

fn roll(sides: i8, modifier: i8) -> i8 {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, sides+1); //lower bound inclusive; upper bound exclusive
    return roll + modifier;
}
