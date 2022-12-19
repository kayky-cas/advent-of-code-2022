use std::str::FromStr;

use anyhow::Result;

struct Forest {
    trees: Vec<Vec<u8>>
}

impl Forest {
    fn new() -> Self {
        Self { trees: vec![] }
    }

    fn hight_trees(&self) -> Vec<i32> {
        let mut trees = vec![];

        let tree_hight = self.trees.len();

        for (i, row) in (&self.trees).iter().enumerate() {

            for (j, tree) in (&row).iter().enumerate() {
                let tree = *tree;
                let tree_width = row.len();

                if i == 0 || i == tree_hight - 1 || j == 0 || j == tree_width - 1 {
                    continue;
                }

                let mut count = 0;

                for n in (0..i).rev() {
                    let current = self.trees[n][j]; 

                    if current >= tree {
                        count += 1;
                        break;
                    }

                    count += 1;
                }

                let mut result = count;
                count = 0;

                for n in i + 1..tree_hight {
                    let current = self.trees[n][j]; 

                    if current >= tree {
                        count += 1;
                        break;
                    }

                    count += 1;
                }

                result *= count;
                count = 0;

                for n in (0..j).rev() {
                    let current = self.trees[i][n]; 

                    if current >= tree {
                        count += 1;
                        break;
                    }

                    count += 1;
                }

                result *= count;
                count = 0;

                for n in j + 1..tree_width {
                    let current = self.trees[i][n]; 

                    if current >= tree {
                        count += 1;
                        break;
                    }

                    count += 1;
                }


                trees.push(result * count);
            }
        }

        trees
    }

    fn get_visibles_trees(&self) -> Vec<u8> {
        let mut trees = vec![];

        let tree_hight = self.trees.len();

        for (i, row) in (&self.trees).iter().enumerate() {

            for (j, tree) in (&row).iter().enumerate() {
                let tree = *tree;
                let tree_width = row.len();

                if i == 0 || i == tree_hight - 1 || j == 0 || j == tree_width - 1 {
                    trees.push(tree);
                    continue;
                }

                let mut found = false;

                for n in 0..i {
                    let current = self.trees[n][j];

                    if current >= tree {
                        found = true;
                        break;
                    }
                }

                if !found {
                    trees.push(tree);
                    continue;
                }

                found = false;

                for n in i + 1..tree_hight {
                    let current = self.trees[n][j];

                    if current >= tree {
                        found = true;
                        break;
                    }
                }

                if !found {
                    trees.push(tree);
                    continue;
                }

                found = false;

                for n in 0..j {
                    let current = self.trees[i][n];

                    if current >= tree {
                        found = true;
                        break;
                    }
                }

                if !found {
                    trees.push(tree);
                }

                found = false;

                for n in j + 1..tree_width {
                    let current = self.trees[i][n];

                    if current >= tree {
                        found = true;
                        break;
                    }
                }

                if !found {
                    trees.push(tree);
                }
            }
        }

        trees
    }
}

impl FromStr for Forest {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut forest = Forest::new();

        s.lines()
            .for_each(|line| {
                let list: Vec<u8> = line
                    .chars()
                    .map(|c| c.to_string()
                    .parse::<u8>().expect("????????"))
                    .collect();

                forest.trees.push(list);
            });

        Ok(forest)
    } 
}

fn main() -> Result<()> {
    let input = include_str!("../inputs/8.prod")
        .parse::<Forest>().unwrap();

    let visibles = input.get_visibles_trees();

    let part1 = visibles.len();
    let mut part2 = input.hight_trees();

    part2.sort();

    println!("{:?}", part1);
    println!("{:?}", part2.last());

    Ok(())
}
