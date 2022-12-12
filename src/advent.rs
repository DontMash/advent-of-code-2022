pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub trait AdventChallenge {
    fn solve_one(&self) -> String;
    fn solve_two(&self) -> String;
}

impl dyn AdventChallenge + '_ {
    pub fn solve(&self) -> (String, String) {
        return (self.solve_one(), self.solve_two());
    }
}
