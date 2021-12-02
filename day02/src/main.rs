use itertools::Itertools;
use std::cmp::max;
use std::io::{self, Read};
use std::str::FromStr;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let (cmd, val) = input.splitn(2, ' ').collect_tuple().ok_or(anyhow::anyhow!(
            "The input '{}' does not have 2 parts",
            input
        ))?;
        let parsed_val: i32 = str::parse(val)?;
        let result = match cmd {
            "forward" => Command::Forward(parsed_val),
            "up" => Command::Up(parsed_val),
            "down" => Command::Down(parsed_val),
            _ => return Err(anyhow::anyhow!("The input '{}' has unknown command", input)),
        };
        Ok(result)
    }
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let commands = input
        .trim()
        .split('\n')
        .map(str::parse)
        .collect::<Result<Vec<Command>, _>>()?;

    part1(&commands);
    part2(&commands);

    Ok(())
}

fn part1(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut pos = 0;
    for command in commands {
        match command {
            &Command::Forward(val) => pos += val,
            &Command::Down(val) => depth += val,
            &Command::Up(val) => depth = max(0, depth - val),
        }
    }
    println!("{}", pos * depth);
}

fn part2(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            &Command::Forward(val) => {
                pos += val;
                depth = max(depth + aim * val, 0);
            }
            &Command::Down(val) => aim += val,
            &Command::Up(val) => aim -= val,
        }
    }
    println!("{}", pos * depth);
}
