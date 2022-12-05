use crate::advent::AdventChallenge;

pub struct Puzzle {
    pub input: String,
}

impl Puzzle {
    fn parse(&self) -> Vec<i32> {
        let entries: Vec<&str> = self
            .input
            .split("\n")
            .filter(|entry| !entry.is_empty())
            .collect();
        let data: Vec<i32> = (0..10).collect();

        return data;
    }
}

impl AdventChallenge for Puzzle {
    /**
     * Puzzle one description
     */
    fn solve_one(&self) -> String {
        let data = self.parse();
        let result = "result";
        return result.to_string();
    }

    /**
     * Puzzle two description
     */
    fn solve_two(&self) -> String {
        let data = self.parse();
        let result = "result";
        return result.to_string();
    }
}
