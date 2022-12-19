use std::str::FromStr;

use anyhow::{Result, Ok, Error};

struct Session {
    max: usize,
    min: usize
}

impl Session {
    fn fully_contains(&self, b: &Session) -> bool {
        return self.min <= b.min && self.max >= b.max
    }

    fn overlap(&self, b: &Session) -> bool {
        (self.min <= b.min && b.min <= self.max) || (self.max >= b.max && b.max >= self.min) 
    }
}

impl FromStr for Session {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_once("-") {
            Some((min, max)) => {
                let min = min.parse().unwrap();
                let max = max.parse().unwrap();

                Ok(Session { max, min })
            }, 
            None => unreachable!("Oh my god how you got here????????")
        }
    }
}


fn main() -> Result<()> {
    let total: Vec<(Session, Session)> = include_str!("../inputs/4.prod")
        .lines()
        .map(|x| {
            match x.split_once(",") {
                Some((a,b)) => {
                    let a = a.parse().unwrap();
                    let b = b.parse().unwrap();

                    (a, b)
                },
                None => unreachable!("WTF!!!!!!")
            }
        }).collect();

    let part1: usize = total.iter()
        .filter(|&(a, b)| a.fully_contains(b) || b.fully_contains(a))
        .count();

    let part2: usize = total.iter()
        .filter(|&(a,b)| a.overlap(b) || b.overlap(a))
        .count();

    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);

    Ok(())
}
