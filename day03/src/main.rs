use anyhow::Result;
use itertools::enumerate;
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums: Vec<&str> = input.split_whitespace().collect();

    part1(&nums);
    part2(&nums)?;
    Ok(())
}

fn part1(nums: &Vec<&str>) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut zero_counts: Vec<i32> = vec![0; nums[0].len()];
    for num in nums {
        for (i, bit) in enumerate(num.chars()) {
            if bit == '0' {
                zero_counts[i] += 1;
            }
        }
    }
    for zero_count in zero_counts {
        gamma *= 2;
        epsilon *= 2;
        if zero_count * 2 > nums.len() as i32 {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }
    println!("{}", gamma * epsilon)
}

fn part2(nums: &Vec<&str>) -> Result<()> {
    fn get_rating<F>(nums: &Vec<&str>, cmp: F) -> Result<i32, ParseIntError>
    where
        F: Fn(usize, usize) -> bool,
    {
        let mut current: BTreeMap<_, _> = nums.iter().map(|x| (x, x.chars())).collect();
        while current.len() != 1 {
            let mut zeros = BTreeMap::new();
            let mut ones = BTreeMap::new();
            for (num, mut chars) in current.into_iter() {
                if chars.next().unwrap() == '0' {
                    zeros.insert(num, chars);
                } else {
                    ones.insert(num, chars);
                }
            }
            if cmp(zeros.len(), ones.len()) {
                current = zeros;
            } else {
                current = ones;
            }
        }
        i32::from_str_radix(current.keys().next().unwrap(), 2)
    }
    println!(
        "{}",
        get_rating(nums, |a, b| a <= b)? * get_rating(nums, |a, b| a > b)?
    );
    Ok(())
}
