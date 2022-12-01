use crate::advent::AdventChallenge;

pub struct Puzzle {
    pub input: String,
}

impl Puzzle {
    fn parse(&self) -> Vec<i32> {
        let entries: Vec<&str> = self.input.split("\n\n").collect();
        let data: Vec<i32> = entries
            .iter()
            .map(|&entry| {
                let items: Vec<&str> = entry.split("\n").collect();
                let item: Vec<i32> = items
                    .iter()
                    .filter(|x| !x.is_empty())
                    .map(|&item| item.parse::<i32>().unwrap())
                    .collect();
                return item.iter().sum();
            })
            .collect();

        return data;
    }
}

impl AdventChallenge for Puzzle {
    /**
     * In case the Elves get hungry and need extra snacks, 
     * they need to know which Elf to ask: they'd like to 
     * know how many Calories are being carried by the Elf 
     * carrying the most Calories. In the example above, 
     * this is 24000 (carried by the fourth Elf). 
     * 
     * Find the Elf carrying the most Calories. How many 
     * total Calories is that Elf carrying?
     */
    fn solve_one(&self) -> String {
        let elves = self.parse();
        let result = elves.iter().max().unwrap();
        return result.to_string();
    }
    
    /**
     * By the time you calculate the answer to the Elves' 
     * question, they've already realized that the Elf 
     * carrying the most Calories of food might eventually 
     * run out of snacks.
     * 
     * To avoid this unacceptable situation, the Elves 
     * would instead like to know the total Calories 
     * carried by the top three Elves carrying the most 
     * Calories. That way, even if one of those Elves runs 
     * out of snacks, they still have two backups.
     * 
     * Find the top three Elves carrying the most Calories. 
     * How many Calories are those Elves carrying in total?
     */
    fn solve_two(&self) -> String {
        let mut elves = self.parse();
        elves.sort_by(|a, b| b.cmp(a));
        return (elves[0] + elves[1] + elves[2]).to_string();
    }
}
