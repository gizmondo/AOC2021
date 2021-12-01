use anyhow::Result;
use itertools::izip;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let depths = input.split_whitespace().map(str::parse::<i32>).collect::<Result<Vec<i32>, _>>()?;

    part1(&depths);
    part2(&depths);
    Ok(())
}

fn count_increasing(seq: &Vec<i32>) -> i32 {
    let mut result = 0;
    for (prev, next) in izip!(seq, &seq[1..]) {
        if next > prev {
            result += 1;
        }
    }
    result
}

fn part1(depths: &Vec<i32>) {
    println!("{}", count_increasing(depths));
}

fn part2(depths: &Vec<i32>)  {
    let windows: Vec<_> = izip!(depths, &depths[1..], &depths[2..]).map(|(a, b, c)| a + b + c).collect();
    println!("{}", count_increasing(&windows));
}