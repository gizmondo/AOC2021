use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::io::{self, Read};
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self {
            x: -self.x,
            y: -self.y,
        };
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Point, Self::Err> {
        let (x, y) = input
            .split(',')
            .map(str::parse::<i32>)
            .collect_tuple()
            .ok_or(anyhow!("The input '{}' is not in for 'x,y' format", input))?;
        Ok(Point { x: x?, y: y? })
    }
}

struct Vent {
    first: Point,
    last: Point,
}

impl Vent {
    fn get_all_points(&self) -> Vec<Point> {
        let diff = self.last - self.first;
        let grad = Point {
            x: diff.x.signum(),
            y: diff.y.signum(),
        };
        let mut current = self.first;
        let mut result = Vec::new();
        result.push(current);
        while current != self.last {
            current = current + grad;
            result.push(current);
        }
        result
    }
}

impl FromStr for Vent {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Vent, Self::Err> {
        let (first, last) = input
            .split(" -> ")
            .map(str::parse)
            .collect_tuple()
            .ok_or(anyhow!(
                "The input '{}' does not contain two points divided by '->'",
                input
            ))?;
        Ok(Vent {
            first: first?,
            last: last?,
        })
    }
}

fn count_overlaps<'a>(vents: impl Iterator<Item=&'a Vent>) {
    let mut count = HashMap::new();
    for vent in vents {
        for point in vent.get_all_points() {
            *count.entry(point).or_insert(0) += 1;
        }
    }
    println!{"{}", count.values().filter(|&v| *v > 1).count()};
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let vents = input
        .trim()
        .split('\n')
        .map(str::parse)
        .collect::<Result<Vec<Vent>, _>>()?;

    count_overlaps(vents.iter().filter(|&vent| vent.first.x == vent.last.x || vent.first.y == vent.last.y));
    count_overlaps(vents.iter());

    Ok(())
}
