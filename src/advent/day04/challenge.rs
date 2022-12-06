use crate::advent::AdventChallenge;

pub struct Puzzle {
    pub input: String,
}

impl Puzzle {
    fn parse(&self) -> Vec<(&str, &str)> {
        let data: Vec<(&str, &str)> = self
            .input
            .split("\n")
            .filter(|entry| !entry.is_empty())
            .map(|entry| entry.split(",").collect::<Vec<&str>>())
            .map(|entry| (entry[0], entry[1]))
            .collect();

        return data;
    }

    fn get_section_range(&self, span: &str) -> Vec<i32> {
        let indices: Vec<i32> = span
            .split("-")
            .map(|item| item.parse::<i32>().unwrap())
            .collect();
        let range_start = indices[0];
        let range_end = indices[1] + 1;

        return (range_start..range_end).collect();
    }
}

impl AdventChallenge for Puzzle {
    /**
     * Space needs to be cleared before the last supplies can be unloaded from
     * the ships, and so several Elves have been assigned the job of cleaning
     * up sections of the camp. Every section has a unique ID number, and each
     * Elf is assigned a range of section IDs.
     *
     * However, as some of the Elves compare their section assignments with
     * each other, they've noticed that many of the assignments overlap. To
     * try to quickly find overlaps and reduce duplicated effort, the Elves
     * pair up and make a big list of the section assignments for each pair
     * (your puzzle input).
     *
     * Some of the pairs have noticed that one of their assignments fully
     * contains the other. For example, 2-8 fully contains 3-7, and 6-6 is
     * fully contained by 4-6. In pairs where one assignment fully contains
     * the other, one Elf in the pair would be exclusively cleaning sections
     * their partner will already be cleaning, so these seem like the most in
     * need of reconsideration.
     * In how many assignment pairs does one range fully contain the other?
     */
    fn solve_one(&self) -> String {
        let pairs = self.parse();

        let results: Vec<i32> = pairs
            .iter()
            .map(|pair| {
                let section_range_one = self.get_section_range(pair.0);
                let section_range_two = self.get_section_range(pair.1);

                if section_range_one.len() > section_range_two.len() {
                    let fully_contains = section_range_two
                        .iter()
                        .all(|item| section_range_one.contains(item));
                    return fully_contains as i32;
                } else {
                    let fully_contains = section_range_one
                        .iter()
                        .all(|item| section_range_two.contains(item));
                    return fully_contains as i32;
                }
            })
            .collect();

        let result: i32 = results.iter().sum();
        return result.to_string();
    }

    /**
     * It seems like there is still quite a bit of duplicate work planned. 
     * Instead, the Elves would like to know the number of pairs that overlap 
     * at all.
     * In how many assignment pairs do the ranges overlap?
     */
    fn solve_two(&self) -> String {
        let pairs = self.parse();

        let results: Vec<i32> = pairs
            .iter()
            .map(|pair| {
                let section_range_one = self.get_section_range(pair.0);
                let section_range_two = self.get_section_range(pair.1);

                if section_range_one.len() > section_range_two.len() {
                    let fully_contains = section_range_two
                        .iter()
                        .any(|item| section_range_one.contains(item));
                    return fully_contains as i32;
                } else {
                    let fully_contains = section_range_one
                        .iter()
                        .any(|item| section_range_two.contains(item));
                    return fully_contains as i32;
                }
            })
            .collect();

        let result: i32 = results.iter().sum();
        return result.to_string();
    }
}
