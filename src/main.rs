mod stats;
#[path = "./utils/json/jsonFile.rs"]
mod json_file;
#[path = "./utils/json/statsJson.rs"]
mod stats_file;


use stats::Stats;
use rand::Rng;
use std::{io};

#[derive(Debug, PartialEq, PartialOrd)]
#[allow(dead_code)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

fn get_choice_from_i8(choice: i8) -> Choice {
    match choice {
        0 => Choice::ROCK,
        1 => Choice::PAPER,
        2 => Choice::SCISSORS,
        _ => panic!("Invalid choice!")
    }
}

fn gen_rand_choice() -> i8 {
    return rand::thread_rng().gen_range(0..=2);
}

fn compare_choice(choice: Choice, stats: &mut Stats) -> () {
    let computed_choice = get_choice_from_i8(gen_rand_choice());
    match computed_choice {
        computed_choice if (computed_choice == Choice::ROCK && choice == Choice::SCISSORS)
        || (computed_choice == Choice::PAPER && choice == Choice::ROCK) || (computed_choice == Choice::SCISSORS && choice == Choice::PAPER) => {
            println!("You lost! I chose {:?}, you chose {:?}", computed_choice, choice);
            stats.inc_losses();
        },
        computed_choice if (choice == Choice::ROCK && computed_choice == Choice::SCISSORS)
        || (choice == Choice::PAPER && computed_choice == Choice::ROCK) || (choice == Choice::SCISSORS && computed_choice == Choice::PAPER)  => {
            println!("You won! I chose {:?}, you chose {:?}", computed_choice, choice);
            stats.inc_wins();
        },
        computed_choice if computed_choice == choice => {
            println!("We drew! I chose {:?}, you chose {:?}", computed_choice, choice);
            stats.inc_draws();
        },
        _ => panic!("Somehow, I chose an option out of the range of choices?")
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stats = Stats::new();

    // Game loop
    loop {
        let mut user_input = String::new();
        stdin.read_line(&mut user_input)?;
        let input = user_input.trim().parse::<i8>().unwrap();
        match input {
            0..=2 => compare_choice(get_choice_from_i8(input), &mut stats),
            3 => stats.print(),
            -1 => return Ok(()),
            _ => println!("Invalid input!"),
        }
    }
}
