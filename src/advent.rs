pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub trait AdventChallenge {
    fn solve_one(&self) -> String;
    fn solve_two(&self) -> String;
}
