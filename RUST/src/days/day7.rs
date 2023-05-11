use std::{collections::HashMap, fs};

fn solution(input: &str) -> (String, String) {
    let cmds: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let dir_sizes: Vec<usize> = dir_sizes(cmds);

    let total_size = 100_000;
    let disk_space = 70_000_000;
    let unused_space_needed = 30_000_000;

    let current_space = disk_space - dir_sizes.iter().last().unwrap();

    let part1 = dir_sizes
        .iter()
        .filter(|&&size| size <= total_size)
        .sum::<usize>()
        .to_string();

    let part2 = dir_sizes
        .iter()
        .find(|&&size| current_space + size >= unused_space_needed)
        .unwrap()
        .to_string();

    (part1, part2)
}

fn dir_sizes(cmds: Vec<Vec<&str>>) -> Vec<usize> {
    let mut stack: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for cmd in cmds {
        match cmd.as_slice() {
            ["$", "cd", "/"] => {
                stack.clear();
                stack.push("root".to_string())
            }
            ["$", "cd", ".."] => {
                stack.pop();
                continue;
            }
            ["$", "cd", folder] => {
                stack.push(folder.to_string());
                continue;
            }
            ["$", "ls"] => continue,
            ["dir", _] => continue,
            [file_size, _] => {
                let file_size_to_number = file_size.parse::<usize>().unwrap();

                for i in 0..stack.len() {
                    let path = stack[..=i].join("/");

                    dir_sizes
                        .entry(path.to_string())
                        .and_modify(|size| *size += file_size_to_number)
                        .or_insert(file_size_to_number);
                }

                continue;
            }

            _ => (),
        }
    }

    let mut dir_sizes_to_vec: Vec<usize> = dir_sizes.values().cloned().collect();
    dir_sizes_to_vec.sort();

    dir_sizes_to_vec
}

pub fn run() {
    let input = fs::read_to_string("../inputs/day7").unwrap();

    let test = solution(&input);

    println!("part1: {}", test.0);
    println!("part2: {}", test.1)
}
