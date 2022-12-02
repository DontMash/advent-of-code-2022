pub mod day01;
pub mod day02;

pub trait AdventChallenge {
    fn solve_one(&self) -> String;
    fn solve_two(&self) -> String;
}
