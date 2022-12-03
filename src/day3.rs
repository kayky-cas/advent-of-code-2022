use anyhow::{Result, Error};
use std::str::FromStr;

struct Ruckstack {
    common: usize
}

fn ascii_to_alph(mut c: usize) -> usize {
    if c >= 97 && c <= 'z' as usize {
        c = c - 96;
    } else {
        c = c - 64 + 26;
    }

    return c;
}

impl FromStr for Ruckstack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_at(s.len() / 2); 

        let mut common: usize = left.chars()
            .find(|&l| right.chars()
                  .find(|&r| l == r)
                  .is_some()).unwrap() as usize;

        common = ascii_to_alph(common);

        Ok(Ruckstack { common })
    }
}

fn common_char(mut s: Vec<&str>) -> usize {
    let str = s.remove(0);

    str.chars()
        .find(|&c| {
            let mut unique = true;
            s.iter().for_each(|x| {
                if x.chars().find(|&f| f == c).is_none() {
                    unique = false;
                }
            });
            return unique;
        }).unwrap() as usize
}

fn main() -> Result<()> {
    let input = include_str!("../inputs/3.prod");

    let part1:usize = input.lines()
        .map(|x| {
            let r = x.parse::<Ruckstack>().unwrap();
            return r.common;
        })
        .sum();

    let part2: usize = input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| ascii_to_alph(common_char(c.to_vec())))
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    return Ok(());
}
