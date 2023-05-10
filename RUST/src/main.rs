use std::{collections::HashMap, fs};

fn solution(input: &str) -> String {
    let cmds: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let dir_sizes: Vec<usize> = dir_sizes(cmds);

    let part1 = dir_sizes
        .iter()
        .filter(|&&size| size <= 100_000)
        .sum::<usize>()
        .to_string();

    part1
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

    let dir_sizes_to_vec: Vec<usize> = dir_sizes.values().cloned().collect();

    dir_sizes_to_vec
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let test = solution(&input);

    println!("{}", test)
}
