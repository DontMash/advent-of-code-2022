use crate::advent::AdventChallenge;

pub struct Puzzle {
    pub input: String,
}

impl Puzzle {
    fn parse(&self) -> String {
        let data = self
            .input
            .chars()
            .filter(|char| !"\n".contains(*char))
            .collect();
        return data;
    }

    fn get_marker_index(&self, data: &String, offset: usize) -> usize {
        let marker_index = data
            .chars()
            .enumerate()
            .map(|(i, _char)| {
                let seq: Vec<char> = data.chars().skip(i).take(offset).collect();
                let mut seq_dedup: Vec<char> = seq.clone();

                seq_dedup.sort();
                seq_dedup.dedup();
                return seq.len() == seq_dedup.len();
            })
            .position(|value| value == true)
            .unwrap();

            return marker_index + offset;
    }
}

impl AdventChallenge for Puzzle {
    /**
     * The preparations are finally complete; you and the Elves leave camp on
     * foot and begin to make your way toward the star fruit grove.
     *
     * As you move through the dense undergrowth, one of the Elves gives you a
     * handheld device. He says that it has many fancy features, but the most
     * important one to set up right now is the communication system.
     *
     * However, because he's heard you have significant experience dealing
     * with signal-based systems, he convinced the other Elves that it would
     * be okay to give you their one malfunctioning device - surely you'll
     * have no problem fixing it.
     *
     * As if inspired by comedic timing, the device emits a few colorful
     * sparks.
     *
     * To be able to communicate with the Elves, the device needs to lock on
     * to their signal. The signal is a series of seemingly-random characters
     * that the device receives one at a time.
     *
     * To fix the communication system, you need to add a subroutine to the
     * device that detects a start-of-packet marker in the datastream. In the
     * protocol being used by the Elves, the start of a packet is indicated by
     * a sequence of four characters that are all different.
     *
     * The device will send your subroutine a datastream buffer (your puzzle
     * input); your subroutine needs to identify the first position where the
     * four most recently received characters were all different.
     * Specifically, it needs to report the number of characters from the
     * beginning of the buffer to the end of the first such four-character
     * marker.
     *
     * How many characters need to be processed before the first
     * start-of-packet marker is detected?
     */
    fn solve_one(&self) -> String {
        let data = self.parse();

        let result = self.get_marker_index(&data, 4);
        return result.to_string();
    }

    /**
     * Your device's communication system is correctly detecting packets, but 
     * still isn't working. It looks like it also needs to look for messages.
     * 
     * A start-of-message marker is just like a start-of-packet marker, except 
     * it consists of 14 distinct characters rather than 4.
     * 
     * How many characters need to be processed before the first 
     * start-of-message marker is detected?
     */
    fn solve_two(&self) -> String {
        let data = self.parse();

        let result = self.get_marker_index(&data, 14);
        return result.to_string();
    }
}
