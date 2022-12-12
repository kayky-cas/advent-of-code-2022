use std::{rc::Rc, cell::RefCell, str::FromStr};

use anyhow::Result;

enum FSEnty {
    Dir,
    File(usize)
}

type Link = Rc<RefCell<Node>>;

struct Node {
    name: String,
    value: FSEnty,
    children: Vec<Link>,
    parent: Option<Link>
}

impl Node {
    fn new_dir(name: String) -> Self {
        Self { name, value: FSEnty::Dir, children: vec![], parent: None }
    }

    fn new_file(name: String, size: usize) -> Self {
        Self { name, value: FSEnty::File(size), children: vec![], parent: None }
    }

    fn append(&mut self, child: Link) {
        self.children.push(child);
    }
}

struct FileSystem {
    root: Link,
    cursor: Link
}

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Node::new_dir("/".to_string())));
        Self { cursor: Rc::clone(&root), root }
    }


    fn cd(&mut self, path: &str) {
        if path == "/" {
            self.cursor = Rc::clone(&self.root);
            return;
        }

        if path == ".." {
            let clone = Rc::clone(&self.cursor.borrow().parent.as_ref().unwrap());
            self.cursor = clone;
            return;
        }

        let mut cd_child: Option<Link> = None;

        for child in &self.cursor.try_borrow_mut().unwrap().children {
            if child.borrow().name == path {
                cd_child = Some(Rc::clone(&child));
                break;
            } 
        }

        self.cursor = cd_child.unwrap();
    }

    fn mkdir(&mut self, name: &str) {
        let child = Rc::new(RefCell::new(Node::new_dir(name.to_string())));

        self.cursor
            .try_borrow_mut().unwrap()
            .append(Rc::clone(&child));

        child.try_borrow_mut().unwrap().parent = Some(Rc::clone(&self.cursor));
    }

    fn touch(&mut self, name: &str, size: usize) {
        let child = Rc::new(RefCell::new(Node::new_file(name.to_string(), size)));

        self.cursor
            .try_borrow_mut().unwrap()
            .append(Rc::clone(&child));

        child.try_borrow_mut().unwrap().parent = Some(Rc::clone(&self.cursor));
    }

    fn print_node(&self, current: Link, spaces: String) {
        let name = &current.borrow().name; 

        println!("{}{}", spaces, name);
        
        for child in &current.borrow().children {
            self.print_node(Rc::clone(&child), spaces.clone() + "   ")
        }
    }

    fn print(&self) {
       self.print_node(Rc::clone(&self.root), "".to_string()); 
    }

    fn size(&self, current: Link) -> usize {
        let value = &current.borrow().value;
        let mut size = 0;

        match value {
            FSEnty::Dir => {
                for child in &current.borrow().children {
                    size += self.size(Rc::clone(&child));
                }
            },
            FSEnty::File(f_size) => {
                size += f_size;
            }
        };

        return size;
    }

    fn size_part1(&self, current: Link, sizes: &mut Vec<usize>) -> usize {
        let value = &current.borrow().value;
        let mut size = 0;

        match value {
            FSEnty::Dir => {
                for child in &current.borrow().children {
                    size += self.size_part1(Rc::clone(&child), sizes);
                }

                if size <= 100000 {
                    sizes.push(size);
                }
            },
            FSEnty::File(f_size) => {
                size += f_size;
            }
        };

        return size;
    }

    fn part1(&self) -> usize {
        let mut sizes: Vec<usize> = vec![];
        self.size_part1(Rc::clone(&self.root), &mut sizes);

        sizes
            .iter()
            .sum()

    }

    fn total_size(&self) -> usize {
        self.size(Rc::clone(&self.root))
    }

    fn delete_part2(&self, current: Link, sizes: &mut Vec<usize>, max: usize) -> usize {
        let value = &current.borrow().value;
        let mut size = 0;

        match value {
            FSEnty::Dir => {
                for child in &current.borrow().children {
                    size += self.delete_part2(Rc::clone(&child), sizes, max);
                }

                if size > max {
                    sizes.push(size);
               }
            },
            FSEnty::File(f_size) => {
                size += f_size;
            }
        };

        return size;
    }

    fn part2(&self) -> usize {
        let need_this = 70000000 - self.total_size();
        0
    }
}

impl FromStr for FileSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut tree = FileSystem::new();

        s.lines()
            .for_each(|s| {
                let command: Vec<_> = s.split(" ").collect();

                if s.starts_with("$") {
                    match command[1] {
                       "cd" => tree.cd(command[2]),
                       _ => {}
                    }
                } else {
                    match command[0] {
                       "dir" => tree.mkdir(command[1]),
                       _ => tree.touch(command[1], command[0].parse::<usize>().unwrap())
                    }
                }
            });

        Ok(tree)
    }
}

fn main() -> Result<()> {
    let tree = include_str!("../inputs/7.prod")
        .parse::<FileSystem>().unwrap();

    tree.print();

    println!("Part 1: {}", tree.part1());

    Ok(())
}
