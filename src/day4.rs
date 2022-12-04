use std::str::FromStr;

use anyhow::{Result, Ok, Error};

struct Session {
    max: usize,
    min: usize
}

impl FromStr for Session {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (min, max) = if let Some((min, max)) = s.split_once("-") {
            let min: usize = min.parse().unwrap();
            let max: usize = max.parse().unwrap();

            (min, max)
        } else {
            unreachable!("Oh my god how you got here????????");
        };

        Ok(Session { max, min })
    }
}

fn overlap(a: &Session, b: &Session) -> bool {
    (a.min <= b.min && b.min <= a.max) || (a.max >= b.max && b.max >= a.min) 
}

fn main() -> Result<()> {
    let total = include_str!("../inputs/4.prod")
        .lines()
        .map(|x| {
            match x.split_once(",") {
                Some((a,b)) => {
                    let a: Session = a.parse().unwrap();
                    let b: Session = b.parse().unwrap();

                    (a, b)
                },
                None => unreachable!("WTF!!!!!!")
            }
        }).collect::<Vec<(Session, Session)>>();

    let part1: usize = total.iter()
        .filter(|&(a, b)| (a.min <= b.min && a.max >= b.max) || (b.min <= a.min && b.max >= a.max))
        .count();

    let part2: usize = total.iter()
        .filter(|&(a,b)| overlap(a, b) || overlap(b, a))
        .count();

    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);

    Ok(())

}
