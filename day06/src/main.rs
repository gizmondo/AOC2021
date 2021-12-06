use anyhow::Result;
use std::io::{self, Read};


fn solve(ages: &Vec<usize>, days: i32) {
    let mut buckets = [0 as i64; 9];
    for &age in ages {
        buckets[age] += 1;
    }

    for _day in 0..days {
        let spowning = buckets[0];
        for age in 1..9 {
            buckets[age - 1] = buckets[age];
        }
        buckets[6] += spowning;
        buckets[8] = spowning;
    }
    println!("{}", buckets.iter().sum::<i64>());
}


fn main() -> Result<()>  {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ages = input
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<usize>, _>>()?;

    solve(&ages, 80);
    solve(&ages, 256);
    
    Ok(())
}
