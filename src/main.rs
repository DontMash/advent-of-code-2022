mod advent;
mod day01;

use crate::advent::AdventChallenge;
use std::{env, fs};

fn main() {
    let argument = env::args().nth(1).expect("No index provided");
    let challenge_index: i32 = argument.parse::<i32>().unwrap();

    match challenge_index {
        1 => {
            let input = fs::read_to_string("./src/day01/input.txt")
                .expect("Failed to read input file day01");
            let day01: day01::challenge::Puzzle = day01::challenge::Puzzle { input };
            let result_one: String = day01.solve_one();
            println!("Result One: {result_one}");
            let result_two: String = day01.solve_two();
            println!("Result Two: {result_two}")
        }

        _ => {
            println!("Unknown challenge index");
        }
    }
}
