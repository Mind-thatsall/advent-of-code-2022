use std::{fs, vec};

use nom::{branch::alt, bytes::streaming::tag, IResult};

enum Operation {
    Cd(Cd),
    Ls(Vec<String>),
}

enum Cd {
    Out,
    In(String),
}

enum Files {
    File { size: usize, name: String },
    Dir(String),
}

impl Operation {
    fn ls(line: &str) {
        println!("{}", line)
    }

    fn cd(line: &str) {
        if line.len() != 0 {
            println!("in a directory")
        } else {
        }
    }
}

fn parse(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ")(input)?;
    let (input, cmd) = alt((tag("ls"), tag("cd")))(input)?;

    match cmd {
        "ls" => Operation::ls(input),
        "cd" => Operation::cd(input),
        _ => println!("unknown file type"),
    }

    Ok((input, Operation::Ls(vec![])))
}

fn main() {
    // read file
    let file = fs::read_to_string("input").unwrap();

    file.lines().for_each(|line| match parse(line) {
        Ok(line) => (),
        Err(err) => println!("Error {}", err),
    })
}
