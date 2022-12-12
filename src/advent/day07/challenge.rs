use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

use crate::advent::AdventChallenge;

pub struct Puzzle {
    pub input: String,
}

impl Puzzle {
    fn parse(&self) -> String {
        let data = self.input.trim().to_owned();

        return data;
    }
}

#[derive(Debug, Clone)]
enum Pointer {
    File(String, u32),
    Directory(String),
}

impl From<&str> for Pointer {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(' ').unwrap();
        match left {
            "dir" => Pointer::Directory(right.to_string()),
            _ => Pointer::File(right.to_string(), left.parse().unwrap()),
        }
    }
}

fn create_file_system(input: String) -> HashMap<String, Vec<Pointer>> {
    let mut file_system: HashMap<String, Vec<Pointer>> = HashMap::new();
    let mut cwd: VecDeque<String> = VecDeque::new();
    let commands = input.split("$ ").filter(|entry| !entry.is_empty());

    for entry in commands {
        let mut io = entry.split("\n").filter(|entry| !entry.is_empty());
        let mut input = io.next().unwrap().split(" ");
        let command = input.next().unwrap();
        match command {
            "cd" => {
                let parameter = input.next().unwrap();
                match parameter {
                    ".." => {
                        cwd.pop_back();
                    }
                    "/" => cwd.clear(),
                    parameter => cwd.push_back(parameter.to_string()),
                }
            }
            "ls" => {
                let path = if cwd.is_empty() {
                    String::from("/")
                } else {
                    format!(
                        "{}/",
                        cwd.iter()
                            .fold("".to_string(), |accum, entry| accum + "/" + entry)
                    )
                };

                let mut pointers: Vec<Pointer> = Vec::new();
                while let Some(entry) = io.next() {
                    pointers.push(Pointer::from(entry));
                }

                file_system.insert(path, pointers);
            }

            _ => {
                panic!("Unexpected command!");
            }
        }
    }

    return file_system;
}

fn get_directory_sizes(
    file_system: &HashMap<String, Vec<Pointer>>,
    sizes: &mut HashMap<String, u32>,
    entrypoint: &str,
) -> u32 {
    let mut total_size = 0;
    if let Some(pointers) = file_system.get(entrypoint) {
        for pointer in pointers {
            match pointer {
                Pointer::File(_, size) => total_size += size,
                Pointer::Directory(name) => {
                    total_size += get_directory_sizes(
                        file_system,
                        sizes,
                        &(entrypoint.to_string() + &name.to_owned() + "/"),
                    )
                }
            }
        }
    }
    sizes.insert(entrypoint.to_string(), total_size);
    total_size
}

impl AdventChallenge for Puzzle {
    /**
     * You can hear birds chirping and raindrops hitting leaves as the
     * expedition proceeds. Occasionally, you can even hear much louder sounds
     * in the distance; how big do the animals get out here, anyway?
     *
     * The device the Elves gave you has problems with more than just its
     * communication system. You try to run a system update:
     * $ system-update --please --pretty-please-with-sugar-on-top
     * Error: No space left on device
     *
     * Perhaps you can delete some files to make space for the update?
     *
     * The filesystem consists of a tree of files (plain data) and directories
     * (which can contain other directories or files). The outermost directory
     * is called /. You can navigate around the filesystem, moving into or out
     * of directories and listing the contents of the directory you're
     * currently in.
     *
     * Within the terminal output, lines that begin with $ are commands you
     * executed, very much like some modern computers:
     *  - cd means change directory. This changes which directory is the
     *  current directory, but the specific result depends on the argument:
     *      - cd x moves in one level: it looks in the current directory for
     *      the directory named x and makes it the current directory.
     *      - cd .. moves out one level: it finds the directory that contains
     *      the current directory, then makes that directory the current
     *      directory.
     *      - cd / switches the current directory to the outermost
     *      directory, /.
     *  - ls means list. It prints out all of the files and directories
     *  immediately contained by the current directory:
     *      - 123 abc means that the current directory contains a file named
     *      abc with size 123.
     *      - dir xyz means that the current directory contains a directory
     *      named xyz.
     *
     * Since the disk is full, your first step should probably be to find
     * directories that are good candidates for deletion. To do this, you need
     * to determine the total size of each directory. The total size of a
     * directory is the sum of the sizes of the files it contains, directly or
     * indirectly. (Directories themselves do not count as having any
     * intrinsic size.)
     *
     * Find all of the directories with a total size of at most 100000.
     * What is the sum of the total sizes of those directories?
     */
    fn solve_one(&self) -> String {
        let data = self.parse();

        let file_system = create_file_system(data);
        let mut sizes: HashMap<String, u32> = HashMap::new();
        get_directory_sizes(&file_system, &mut sizes, "/");

        let result = sizes.values().filter(|s| **s <= 100_000).sum::<u32>();
        return result.to_string();
    }

    /**
     * Now, you're ready to choose a directory to delete.
     *
     * The total disk space available to the filesystem is 70000000. To run
     * the update, you need unused space of at least 30000000. You need to
     * find a directory you can delete that will free up enough space to run
     * the update.
     *
     * Find the smallest directory that, if deleted, would free up enough
     * space on the filesystem to run the update. What is the total size of
     * that directory?
     */
    fn solve_two(&self) -> String {
        let data = self.parse();

        let file_system = create_file_system(data);
        let mut sizes: HashMap<String, u32> = HashMap::new();
        get_directory_sizes(&file_system, &mut sizes, "/");

        let needed_space: u32 = 30_000_000;
        let free_space: u32 = 70_000_000 - sizes.get("/").unwrap();
        let need_to_free: u32 = needed_space - free_space;

        let result = *sizes
            .values()
            .sorted()
            .find(|s| **s >= need_to_free)
            .unwrap();
        return result.to_string();
    }
}
