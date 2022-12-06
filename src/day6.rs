use std::str::FromStr;

use anyhow::{Result, Error};

struct Buffer14 {
    marker: usize
}

impl FromStr for Buffer14 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let chunks = s.chars()
            .collect::<Vec<_>>();

        for (i, c) in chunks.windows(14).enumerate() {
            let mut have = false;            
            for (px, ch) in c.iter().enumerate() {
                for px2 in (px + 1)..c.len() {
                    if &c[px2] == ch {
                        have = true;
                        break;
                    }
                    if have {
                        break;
                    }
                }

                if have {
                    break;
                }
            }

            if !have {
                return Ok(Buffer14 { marker: i + 14 });    
            }
        }

        Err(anyhow::anyhow!("Meeeee"))
    }
}

struct Buffer {
    marker: usize
}

impl FromStr for Buffer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let chunks = s.chars()
            .collect::<Vec<_>>();

        for (i, c) in chunks.windows(4).enumerate() {
            let mut have = false;            
            for (px, ch) in c.iter().enumerate() {
                for px2 in (px + 1)..c.len() {
                    if &c[px2] == ch {
                        have = true;
                        break;
                    }
                    if have {
                        break;
                    }
                }

                if have {
                    break;
                }
            }

            if !have {
                return Ok(Buffer { marker: i + 4 });    
            }
        }

        Err(anyhow::anyhow!("Meeeee"))
    }
}

fn main() -> Result<()> {

    let input = include_str!("../inputs/6.prod");

    let part1 = input
        .parse::<Buffer>()
        .unwrap()
        .marker;

    let part2 = input
        .parse::<Buffer14>()
        .unwrap()
        .marker;

    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);

    Ok(())
}
