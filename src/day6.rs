use std::{collections::HashSet, usize};

use anyhow::Result;

fn get_marker(buffer: &str, marker_count: usize) -> usize {
    buffer
        .as_bytes()
        .windows(marker_count)
        .enumerate()
        .filter(|(_, x)| x.iter().collect::<HashSet<&u8>>().len() == marker_count)
        .map(|(i, _)| i)
        .next().unwrap() + marker_count
}

fn main() -> Result<()> {
    let buffer = include_str!("../inputs/6.prod");

    let part1 = get_marker(buffer, 4);
    let part2 = get_marker(buffer, 14);

    println!("Part 1 = {:?}", part1);
    println!("Part 2 = {:?}", part2);

    Ok(())
}
