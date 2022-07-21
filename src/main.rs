use std::io;
use rand::distributions::{Distribution, Uniform};
// use std::cmp::Ordering;

fn main() {
    // death_saves_fail_probability();
    expected_value();
    // guessing_game();
}

fn _death_saves_fail_probability() {

    println!("Enter the number of samples");
    let mut number_of_samples = String::new();
    io::stdin()
        .read_line(&mut number_of_samples)
        .expect("Failed to read line");
    println!("Number of samples: {number_of_samples}");
    let number_of_samples: u32 = number_of_samples.trim().parse().expect("Number is required");

    let mut sum = 0;
    let sample_size = number_of_samples;//100_000_000;
    for _ in 1..=sample_size {
        sum += _attempt_death_save();
    }
    println!("{}", sum as f32 / sample_size as f32);
}
fn _attempt_death_save() -> u32 {
    let mut number_of_saves = 0;
    let mut number_of_fails = 0;
    loop {
        let roll_value = roll(20, RollModifier::Advantage);
        if roll_value == 20 {
            return 1
        } else if roll_value >= 10 {
            number_of_saves += 1;
        } else if roll_value > 1 {
            number_of_fails += 1;
        } else {
            number_of_fails += 2;
        }
        if number_of_saves >= 3 {
            return 1
        } else if number_of_fails >= 3 {
            return 0
        }
    }
}

#[derive(Clone, Copy)]
enum RollModifier {
    DoubleAdvantage,
    Advantage,
    Normal,
    Disadvantage
}

fn roll(sides: u32, modifier: RollModifier) -> u32 {
    let range = Uniform::from(1..=sides);
    let mut rng = rand::thread_rng();
    let value = range.sample(&mut rng);
    match modifier {
        RollModifier::DoubleAdvantage => {
            let second = range.sample(&mut rng);
            let third = range.sample(&mut rng);
            if third > second {
                if third > value {
                    return third
                }
            } else {
                if second > value {
                    return second
                }
            }
        },
        RollModifier::Advantage => {
            let second = range.sample(&mut rng);
            if second > value {
                return second
            }
        },
        RollModifier::Normal => {},
        RollModifier::Disadvantage => {
            let second = range.sample(&mut rng);
            if second < value {
                return second
            }
        }
    }
    value
}

fn expected_value() {
    let mut number_of_sides = String::new();
    println!("Enter the number of sides for the die");
    io::stdin()
        .read_line(&mut number_of_sides)
        .expect("Failed to read line");
    println!("Number of sides: {number_of_sides}");
    let number_of_sides: u32 = number_of_sides.trim().parse().expect("Number is required");

    println!("Enter the roll modifier:");
    println!("1: Double Advantage");
    println!("2: Advantage");
    println!("3: Normal");
    println!("Other: Disadvantage");
    let mut modifier_number = String::new();
    io::stdin()
        .read_line(&mut modifier_number)
        .expect("Failed to read line");
    println!("Number of samples: {modifier_number}");
    let modifier_number: u32 = modifier_number.trim().parse().expect("Number is required");
    let roll_modifier: RollModifier;
    if modifier_number == 1 {
        roll_modifier = RollModifier::DoubleAdvantage;
    } else if modifier_number == 2 {
        roll_modifier = RollModifier::Advantage;
    } else if modifier_number == 3 {
        roll_modifier = RollModifier::Normal;
    } else {
        roll_modifier = RollModifier::Disadvantage;
    }

    println!("Enter the number of samples");
    let mut number_of_samples = String::new();
    io::stdin()
        .read_line(&mut number_of_samples)
        .expect("Failed to read line");
    println!("Number of samples: {number_of_samples}");
    let number_of_samples: u32 = number_of_samples.trim().parse().expect("Number is required");

    let mut sum = 0;
    let sample_size = number_of_samples;//100_000_000;
    for _ in 1..=sample_size {
        sum += roll(number_of_sides, roll_modifier)
    }
    println!("{}", sum as f32 / sample_size as f32);
}

fn _guessing_game() {

    let minimum = 1;
    let maximum = 10;
    println!("Guess the number between {} and {} (bounds included)!", minimum, maximum);

    let range = Uniform::from(minimum..=maximum);
    let mut rng = rand::thread_rng();
    let secret_number = range.sample(&mut rng);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {guess}");
        let guess: u32 = guess.trim().parse().expect("Number is required");
        if guess == secret_number {
            println!("Matched!");
            break;
        } else if guess > secret_number {
            println!("Too High");
        } else {
            println!("Too Low");
        }
    }
}

/*
    ROLL MODIFIERS
        Flat +/-
        Dbl Adv / Adv / Dis
        Inc / Dec dice sides
        Lucky / Pocket roll / Re-Role
        Additional dice (crit, sneak, etc.)





    Attack d4, d6, etc
    Defense d4, d6, etc
    Max_Health 4, 6, etc
    Current_Health ... 
    AGI? cool down of d12, d10, d8, d6, d4, d2, d1?
    Escape... take one "opportunity attack"? []
    Stamina / Energy / ??? (special actions) [OOS] 

    PROFESSIONS
        "Adventurer"
        --- Gathering / Wilderness Professions ---
        - Hunter / Trapper / Tracker
        - Herbalist / Alchemist
        - Lumberjack
        - Miner
        - Explorer
        --- Crafting / City Professions ---
        - Butcher
        - Tanner
        - Healer
        - Carpenter
        - Blacksmith
        - Cartographer

    GUESSING GAME -> choose environment to search
        Skill check?
            Unguarded Loot
            Combat / Loot
            Trap / Treasure
            Point of interest discovered?
            Nothing of interest?

    COMBAT
        Turn based?
        Live?
        ...

*/