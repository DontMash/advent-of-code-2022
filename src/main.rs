mod advent;

use std::{env, fs};

fn solve_challenge(challenge: &dyn advent::AdventChallenge) {
    let result = challenge.solve();
    println!("Result One: {}", result.0);
    println!("Result Two: {}", result.1);
}

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
            solve_challenge(&challenge);
        }
        2 => {
            let challenge = advent::day02::challenge::Puzzle { input };
            solve_challenge(&challenge);
        }
        3 => {
            let challenge = advent::day03::challenge::Puzzle { input };
            solve_challenge(&challenge);
        }
        4 => {
            let challenge = advent::day04::challenge::Puzzle { input };
            solve_challenge(&challenge);
        }
        5 => {
            let challenge = advent::day05::challenge::Puzzle { input };
            solve_challenge(&challenge);
        }
        6 => {
            let challenge = advent::day06::challenge::Puzzle { input };
            solve_challenge(&challenge);
        }
        7 => {
            let challenge = advent::day07::challenge::Puzzle { input };
            solve_challenge(&challenge);
        }

        _ => {
            println!("Unknown challenge index");
        }
    }
}
