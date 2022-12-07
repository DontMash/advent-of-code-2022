use crate::advent::AdventChallenge;

pub struct Puzzle {
    pub input: String,
}

impl Puzzle {
    fn parse(&self) -> Vec<&str> {
        let data: Vec<&str> = self
            .input
            .split("\n")
            .filter(|entry| !entry.is_empty())
            .collect();

        return data;
    }

    fn get_crates(&self, data: &Vec<&str>) -> Vec<Vec<char>> {
        let crate_data: Vec<Vec<String>> = self.get_crate_data(data);
        let mut crates: Vec<Vec<char>> = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        for row in crate_data {
            for (k, crat) in row.iter().enumerate() {
                let value: String = crat
                    .trim()
                    .chars()
                    .filter(|char| !"[]".contains(*char))
                    .collect();
                if value.is_empty() {
                    continue;
                }

                crates[k].push(value.chars().next().unwrap());
            }
        }
        return crates;
    }

    fn get_crate_data(&self, data: &Vec<&str>) -> Vec<Vec<String>>{
        return data
        .iter()
        .take(8)
        .map(|row| {
            row.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|crat| crat.iter().collect::<String>())
                .collect()
        })
        .collect();
    }

    fn get_moves(&self, data: &Vec<&str>) -> Vec<Vec<usize>> {
        return data
        .iter()
        .skip(9)
        .map(|entry| {
            entry
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|item| item.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    }
}

impl AdventChallenge for Puzzle {
    /**
     * The expedition can depart as soon as the final supplies have been
     * unloaded from the ships. Supplies are stored in stacks of marked
     * crates, but because the needed supplies are buried under many other
     * crates, the crates need to be rearranged.
     * The ship has a giant cargo crane capable of moving crates between
     * stacks. To ensure none of the crates get crushed or fall over, the
     * crane operator will rearrange them in a series of carefully-planned
     * steps. After the crates are rearranged, the desired crates will be at
     * the top of each stack.
     *
     * The Elves don't want to interrupt the crane operator during this
     * delicate procedure, but they forgot to ask her which crate will end up
     * where, and they want to be ready to unload them as soon as possible so
     * they can embark.
     *
     * They do, however, have a drawing of the starting stacks of crates and
     * the rearrangement procedure (your puzzle input).
     *
     * The Elves just need to know which crate will end up on top of each
     * stack; in this example, the top crates are C in stack 1, M in stack 2,
     * and Z in stack 3, so you should combine these together and give the
     * Elves the message CMZ.
     * After the rearrangement procedure completes, what crate ends up on top
     * of each stack?
     */
    fn solve_one(&self) -> String {
        let data = self.parse();

        let crates = &mut self.get_crates(&data);
        let moves = self.get_moves(&data);
        for entry in moves {
            let amount = entry[0];
            let source = entry[1] - 1;
            let target = entry[2] - 1;

            let crat: Vec<char> = crates[source].drain(..amount).rev().collect();
            crates[target].splice(0..0, crat);
        }

        let result: String = crates.iter().map(|entry| entry[0]).collect();
        return result;
    }

    /**
     * As you watch the crane operator expertly rearrange the crates, you 
     * notice the process isn't following your prediction.
     * 
     * Some mud was covering the writing on the side of the crane, and you 
     * quickly wipe it away. The crane isn't a CrateMover 9000 - it's a 
     * CrateMover 9001.
     * 
     * The CrateMover 9001 is notable for many new and exciting features: air 
     * conditioning, leather seats, an extra cup holder, and the ability to 
     * pick up and move multiple crates at once.
     * 
     * Before the rearrangement process finishes, update your simulation so 
     * that the Elves know where they should stand to be ready to unload the 
     * final supplies. After the rearrangement procedure completes, what crate 
     * ends up on top of each stack?
     */
    fn solve_two(&self) -> String {
        let data = self.parse();

        let crates = &mut self.get_crates(&data);
        let moves = self.get_moves(&data);
        for entry in moves {
            let amount = entry[0];
            let source = entry[1] - 1;
            let target = entry[2] - 1;

            let crat: Vec<char> = crates[source].drain(..amount).collect();
            crates[target].splice(0..0, crat);
        }

        let result: String = crates.iter().map(|entry| entry[0]).collect();
        return result;
    }
}
