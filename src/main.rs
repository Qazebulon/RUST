extern crate rand;
extern crate regex;

use rand::Rng;
use std::io::{self, Write};
use regex::Regex;

// struct Die {
//     sides: u64,
//     modifier: u64,
// }

// Project Trello: https://trello.com/b/8QqyFxg6/rust

fn main() {
    loop {
        let input = input();
        match input.trim() {
            // "d2+4" => formatted_roll(2,4),
            // "d2" => formatted_roll(2,0),
            // "d4" => formatted_roll(4,0),
            // "d6" => formatted_roll(6,0),
            // "d8" => formatted_roll(8,0),
            // "d10" => formatted_roll(10,0),
            // "d12" => formatted_roll(12,0),
            // "d20" | "r" => formatted_roll(20,0),
            // "d100" => formatted_roll(100,0),
            "q" => break,
            "?" => help(),
            "" => (),
            _ => process_other_input(input)
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

fn formatted_roll(sides: u64, modifier: u64) {
    println!("> You rolled a: {}", roll(sides, modifier));
}

fn roll(sides: u64, modifier: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, sides+1); //lower bound inclusive; upper bound exclusive
    return roll + modifier;
}

fn roll_n_dice(sides: u64, number_of_dice: u64) -> u64 {
    (0..number_of_dice).fold(0, |sum, val| {
        sum + roll(sides, 0)
    })
}

fn process_other_input(input: String) {
    let d_regex = Regex::new("d").unwrap();
    if d_regex.is_match(&input) {
        parse_dice_roll(input);
    } else {
        println!("! Invalid Command (enter ? for help)");
    };
}
fn parse_dice_roll(input: String) {
    let die_regex = Regex::new(r"(\d+)d(\d+)").unwrap();
    for cap in die_regex.captures_iter(&input) {
        let number_of_dice = cap[1].parse::<u64>().unwrap();
        let sides_of_dice = cap[2].parse::<u64>().unwrap();
        let total = roll_n_dice(sides_of_dice, number_of_dice);
        println!("> You rolled a: {}", total);
    }
}
