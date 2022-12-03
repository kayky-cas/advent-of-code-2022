use anyhow::Result;
use std::str::FromStr;


const RESULTS: [usize; 9] = [3, 6, 0, 0, 3, 6, 6, 0, 3];
const RESULTS_PART2: [usize; 9] = [3, 4, 8, 1, 5, 9, 2, 6, 7];

fn convert_num(c: &str) -> usize {
    match c {
        "A" => 0,
        "B" => 3,
        "C" => 6,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("Oh my fucking god how you got here????")
    }
}

struct HandleRPS {
    value: usize
}

impl FromStr for HandleRPS {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (e, m) = match s.split_once(" ") {
            Some((e, m)) => (convert_num(e), convert_num(m)),
            None => return Err(anyhow::anyhow!("input is bad bruh"))
        };

        Ok(HandleRPS { value: (m + 1) + (RESULTS[e + m]) })
    }
}

struct HandleRPS2 {
    value: usize
}

impl FromStr for HandleRPS2 { 
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (e, m) = match s.split_once(" ") {
            Some((e, m)) => (convert_num(e), convert_num(m)),
            None => return Err(anyhow::anyhow!("input is bad bruh"))
        };

        Ok(HandleRPS2 { value: RESULTS_PART2[e + m] })
    }
}
fn main() -> Result<()> {
    let str = include_str!("../inputs/2.prod");

    let play_sums: usize = str
        .lines()
        .map(|play| play.parse::<HandleRPS>().unwrap().value)
        .sum();


    let play_sums_2: usize = str
        .lines()
        .map(|play| play.parse::<HandleRPS2>().unwrap().value)
        .sum();

    println!("Part 1: {}", play_sums);
    println!("Part 2: {}", play_sums_2);

    return Ok(());
}
