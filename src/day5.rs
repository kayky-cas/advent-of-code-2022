use std::{rc::Rc, env::split_paths};

use anyhow::{Result, Ok};

fn main() -> Result<()> {

    let (inst, commands) = include_str!("../inputs/5.prod")
        .split_once("\n\n").unwrap();

    let quant = inst        
        .lines()
        .rev()
        .nth(0)
        .unwrap()
        .trim()
        .chars()
        .rev()
        .nth(0)
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    let inst = inst
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();

            let mut s: Vec<char> = Vec::new();
            let mut pos = 1;

            for _ in 0..quant {
                s.push(chars[pos]); 
                pos += 4;
            }
            s
        });

    let mut stack: Vec<Vec<char>> = Vec::new(); 
    let mut stack_2: Vec<Vec<char>> = Vec::new(); 

    for _ in 0..quant {
        stack.push(Vec::new());
        stack_2.push(Vec::new());
    }

    for x in inst {
        for i in 0..quant {
            if x[i] != ' ' {
                stack[i].push(x[i]);
                stack_2[i].push(x[i]);
            }
        }        
    }

    commands
        .lines()
        .map(|line| {
            let mut u: Vec<usize> = Vec::new();
            
            line.split(" ")
                .for_each(|x| {
                    let x = x.parse::<usize>();
                    
                    if x.is_ok() {
                        u.push(x.unwrap());
                    }
                });
            u 
        }).for_each(|x| {
            let quant = x[0];
            let from = x[1] - 1;
            let to = x[2] - 1;

            let mut list: Vec<char> = Vec::new();

            for _ in 0..quant {
                let pop = stack[from].pop().unwrap();
                let pop2 = stack_2[from].pop().unwrap();
                list.push(pop2);
                stack[to].push(pop); 
            }

            for z in list.iter().rev() {
                stack_2[to].push(*z); 
            }
        });

    print!("Part 1 = ");
    for x in stack {
        print!("{}", &x.last().unwrap());
    }

    println!();

    print!("Part 2 = ");
    for x in stack_2 {
        print!("{}", &x.last().unwrap());
    }

    println!();

    Ok(())
}
