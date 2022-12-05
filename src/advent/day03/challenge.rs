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

    fn prioritize(&self, item: u32) -> u32 {
        if item < 97 {
            return item - 38;
        } else {
            return item - 96;
        }
    }
}

impl AdventChallenge for Puzzle {
    /**
     * One Elf has the important job of loading all of the rucksacks with
     * supplies for the jungle journey. Unfortunately, that Elf didn't quite
     * follow the packing instructions, and so a few items now need to be
     * rearranged.
     *
     * Each rucksack has two large compartments. All items of a given type are
     * meant to go into exactly one of the two compartments. The Elf that did
     * the packing failed to follow this rule for exactly one item type per
     * rucksack.
     *
     * The Elves have made a list of all of the items currently in each
     * rucksack (your puzzle input), but they need your help finding the
     * errors. Every item type is identified by a single lowercase or
     * uppercase letter (that is, a and A refer to different types of items).
     *
     * The list of items for each rucksack is given as characters all on a
     * single line. A given rucksack always has the same number of items in
     * each of its two compartments, so the first half of the characters
     * represent items in the first compartment, while the second half of the
     * characters represent items in the second compartment.
     *
     * To help prioritize item rearrangement, every item type can be converted
     * to a priority:
     * - Lowercase item types a through z have priorities 1 through 26.
     * - Uppercase item types A through Z have priorities 27 through 52.
     * Find the item type that appears in both compartments of each rucksack.
     * What is the sum of the priorities of those item types?
     */
    fn solve_one(&self) -> String {
        let rucksacks = self.parse();

        let results: Vec<u32> = rucksacks
            .iter()
            .map(|items| {
                let half_item_count = items.len() / 2;
                let mut first_compartment = items.chars().take(half_item_count);
                let second_compartment: String =
                    items.chars().rev().take(half_item_count).collect();
                let equal_item = first_compartment
                    .find(|&item| second_compartment.contains(item))
                    .unwrap();
                return equal_item as u32;
            })
            .map(|item| self.prioritize(item))
            .collect();

        let result: u32 = results.iter().sum();
        return result.to_string();
    }

    /**
     * As you finish identifying the misplaced items, the Elves come to you
     * with another issue.
     *
     * For safety, the Elves are divided into groups of three. Every Elf
     * carries a badge that identifies their group. For efficiency, within
     * each group of three Elves, the badge is the only item type carried by
     * all three Elves. That is, if a group's badge is item type B, then all
     * three Elves will have item type B somewhere in their rucksack, and at
     * most two of the Elves will be carrying any other item type.
     *
     * The problem is that someone forgot to put this year's updated
     * authenticity sticker on the badges. All of the badges need to be pulled
     * out of the rucksacks so the new authenticity stickers can be attached.
     *
     * Additionally, nobody wrote down which item type corresponds to each
     * group's badges. The only way to tell which item type is the right one
     * is by finding the one item type that is common between all three Elves
     * in each group.
     *
     * Every set of three lines in your list corresponds to a single group,
     * but each group can have a different badge item type. Priorities for
     * these items must still be found to organize the sticker attachment
     * efforts.
     * Find the item type that corresponds to the badges of each three-Elf
     * group. What is the sum of the priorities of those item types?
     */
    fn solve_two(&self) -> String {
        let rucksacks = self.parse();

        let results: Vec<u32> = rucksacks
            .chunks(3)
            .into_iter()
            .map(|group| {
                let longest_inventory = group
                    .iter()
                    .max_by(|item_a, item_b| item_a.chars().count().cmp(&item_b.chars().count()))
                    .unwrap();
                let equal_item = longest_inventory
                    .chars()
                    .find(|&item| {
                        return group
                            .iter()
                            .fold(true, |accum, group_item| accum && group_item.contains(item));
                    })
                    .unwrap();
                return self.prioritize(equal_item as u32);
            })
            .collect();

        let result: u32 = results.iter().sum();
        return result.to_string();
    }
}
