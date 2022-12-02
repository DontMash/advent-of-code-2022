mod advent;

use crate::advent::AdventChallenge;
use std::{env, fs};

fn main() {
    let argument = env::args().nth(1).expect("No index provided");
    let challenge_index: i32 = argument.parse::<i32>().unwrap();

    let file_index: String = ("0".to_owned() + &argument)
        .chars()
        .rev()
        .take(2)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    let input = fs::read_to_string(format!("./src/advent/day{file_index}/input.txt"))
        .expect(format!("Failed to read input file day{file_index}").as_str());

    match challenge_index {
        1 => {
            let challenge = advent::day01::challenge::Puzzle { input };
            let result_one: String = challenge.solve_one();
            println!("Result One: {result_one}");
            let result_two: String = challenge.solve_two();
            println!("Result Two: {result_two}")
        }
        2 => {
            let challenge = advent::day02::challenge::Puzzle { input };
            let result_one: String = challenge.solve_one();
            println!("Result One: {result_one}");
            let result_two: String = challenge.solve_two();
            println!("Result Two: {result_two}")
        }

        _ => {
            println!("Unknown challenge index");
        }
    }
}
