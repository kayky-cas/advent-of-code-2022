use anyhow::Result;
use std::cmp::Ord;

fn main() -> Result<()> {
    let mut cals = include_str!("../inputs/1.prod")    
        .split("\n\n")
        .map(|cal| {
            return cal.split("\n")
                    .flat_map(|x| x.parse::<usize>())
                    .sum::<usize>();
        }).collect::<Vec<usize>>();

    cals.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", cals[0]);
    println!("Part 2: {:?}", cals.iter().take(3).sum::<usize>());

    return Ok(());
}
