use std::collections::HashMap;

use crate::advent::AdventChallenge;

pub struct Puzzle {
    pub input: String,
}

impl Puzzle {
    fn parse(&self) -> Vec<(String, String)> {
        let entries: Vec<&str> = self
            .input
            .split("\n")
            .filter(|entry| !entry.is_empty())
            .collect();
        let data = entries
            .iter()
            .map(|entry| {
                let items: Vec<&str> = entry.split(" ").collect();
                return (items[0].to_string(), items[1].to_string());
            })
            .collect();

        return data;
    }
}

impl AdventChallenge for Puzzle {
    /**
     * The Elves begin to set up camp on the beach. To
     * decide whose tent gets to be closest to the snack
     * storage, a giant Rock Paper Scissors tournament
     * is already in progress.
     *
     * Appreciative of your help yesterday, one Elf
     * gives you an encrypted strategy guide (your
     * puzzle input) that they say will be sure to help
     * you win. "The first column is what your opponent
     * is going to play: A for Rock, B for Paper, and C
     * for Scissors. The second column--" Suddenly, the
     * Elf is called away to help with someone's tent.
     *
     * The second column, you reason, must be what you
     * should play in response: X for Rock, Y for Paper,
     * and Z for Scissors. Winning every time would be
     * suspicious, so the responses must have been
     * carefully chosen.
     *
     * The winner of the whole tournament is the player
     * with the highest score. Your total score is the
     * sum of your scores for each round. The score for
     * a single round is the score for the shape you
     * selected (1 for Rock, 2 for Paper, and 3 for
     * Scissors) plus the score for the outcome of the
     * round (0 if you lost, 3 if the round was a draw,
     * and 6 if you won).
     *
     * Since you can't be sure if the Elf is trying to
     * help you or trick you, you should calculate the
     * score you would get if you were to follow the
     * strategy guide.
     * What would your total score be if everything goes
     * exactly according to your strategy guide?
     */
    fn solve_one(&self) -> String {
        let indices: HashMap<&str, i32> =
            HashMap::from([("A", 0), ("B", 1), ("C", 2), ("X", 0), ("Y", 1), ("Z", 2)]);
        let matches: [[i32; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
        let games = self.parse();

        let results: Vec<i32> = games
            .iter()
            .map(|game| {
                let opponent_index = indices.get(&game.0 as &str).unwrap();
                let player_index = indices.get(&game.1 as &str).unwrap();
                return matches[*opponent_index as usize][*player_index as usize];
            })
            .collect();
        let result: i32 = results.iter().sum();

        return result.to_string();
    }

    /**
     * The Elf finishes helping with the tent and sneaks 
     * back over to you. "Anyway, the second column says 
     * how the round needs to end: X means you need to lose, 
     * Y means you need to end the round in a draw, and Z 
     * means you need to win. Good luck!"
     * 
     * The total score is still calculated in the same way, 
     * but now you need to figure out what shape to choose 
     * so the round ends as indicated.
     * 
     * Following the Elf's instructions for the second 
     * column, what would your total score be if everything 
     * goes exactly according to your strategy guide?
     */
    fn solve_two(&self) -> String {
        let indices: HashMap<&str, i32> =
            HashMap::from([("A", 0), ("B", 1), ("C", 2), ("X", 0), ("Y", 1), ("Z", 2)]);
        let matches: [[i32; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
        let games = self.parse();

        let results: Vec<i32> = games
            .iter()
            .map(|game| {
                let opponent_index = indices.get(&game.0 as &str).unwrap();
                let player_index = indices.get(&game.1 as &str).unwrap();
                return matches[*opponent_index as usize][*player_index as usize];
            })
            .collect();
        let result: i32 = results.iter().sum();

        return result.to_string();
    }
}
