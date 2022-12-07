use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
enum TerminalCommand {
    Cd(String),
    Ls(Vec<LsOutput>),
}

#[derive(Debug)]
enum LsOutput {
    Dir(String),
    File { size: usize, name: String },
}

impl FromStr for TerminalCommand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let command = lines.next().ok_or("Invalid command")?.trim();

        if command.starts_with("cd") {
            Ok(TerminalCommand::Cd(
                command
                    .split(' ')
                    .nth(1)
                    .ok_or("Invalid command")?
                    .to_string(),
            ))
        } else if command.starts_with("ls") {
            Ok(TerminalCommand::Ls(
                lines
                    .map(str::parse)
                    .collect::<Result<Vec<LsOutput>, _>>()?,
            ))
        } else {
            Err("Invalid command")?
        }
    }
}

impl FromStr for LsOutput {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut command = s.split(' ');
        match (command.next(), command.next()) {
            (Some("dir"), Some(name)) => Ok(LsOutput::Dir(name.to_string())),
            (Some(size), Some(name)) => Ok(LsOutput::File {
                size: size.parse()?,
                name: name.to_string(),
            }),
            _ => Err("Invalid ls ouput")?,
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    Dir {
        name: String,
        children: Vec<Rc<RefCell<Node>>>,
    },
    File {
        size: usize,
        name: String,
    },
}

impl Node {
    fn push_child(&mut self, child: Rc<RefCell<Node>>) {
        if let Node::Dir { children, .. } = self {
            children.push(child)
        }
    }
}

impl Node {
    fn sum_of_dirs_lt(&self, n: usize) -> (usize, usize) {
        let mut sum_of_dirs = 0;
        let mut total = 0;
        if let Self::Dir { children, .. } = self {
            total = children
                .iter()
                .map(|child| match *child.borrow() {
                    Self::Dir { .. } => {
                        let (total, sum) = child.borrow().sum_of_dirs_lt(n);
                        sum_of_dirs += sum;
                        if total <= n {
                            sum_of_dirs += total
                        }
                        total
                    }
                    Self::File { size, .. } => size,
                })
                .sum();
        }
        (total, sum_of_dirs)
    }
}

#[derive(Debug)]
struct FileTree {
    base: Node,
}

impl FileTree {
    fn new(commands: &[TerminalCommand]) -> Self {
        use TerminalCommand::*;
        let mut dir_history: Vec<Rc<RefCell<Node>>> = vec![];
        let mut command_iter = commands.iter();
        let base = Rc::new(RefCell::new(match command_iter.next() {
            Some(Cd(name)) => Node::Dir {
                children: vec![],
                name: name.clone(),
            },
            _ => unreachable!(),
        }));
        let mut current = base.clone();

        for command in command_iter {
            match command {
                Cd(dir) => match &dir[..] {
                    ".." => current = dir_history.pop().unwrap(),
                    name => {
                        dir_history.push(current.clone());
                        let new_dir = Rc::new(RefCell::new(Node::Dir {
                            children: vec![],
                            name: name.to_string(),
                        }));
                        {
                            let mut node = current.borrow_mut();
                            node.push_child(new_dir.clone());
                        }
                        current = new_dir;
                    }
                },
                Ls(outputs) => {
                    for output in outputs {
                        if let LsOutput::File { size, name } = output {
                            let mut node = current.borrow_mut();
                            node.push_child(Rc::new(RefCell::new(Node::File {
                                size: *size,
                                name: name.clone(),
                            })));
                        }
                    }
                }
            }
        }

        Self {
            base: (*base).clone().into_inner(),
        }
    }

    fn sum_of_dirs_lt(&self, n: usize) -> usize {
        let (_, sum) = self.base.sum_of_dirs_lt(n);
        sum
    }
}

#[cfg(feature = "test_input")]
fn get_input() -> Result<String, Box<dyn Error>> {
    Ok("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
        .to_string())
}

#[cfg(not(feature = "test_input"))]
fn get_input() -> Result<String, Box<dyn Error>> {
    use std::fs;

    Ok(fs::read_to_string("inputs/day07input.txt")?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let commands = input
        .split('$')
        .filter(|c| !c.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<TerminalCommand>, _>>()?;
    let file_tree = FileTree::new(&commands);
    let result_part1 = file_tree.sum_of_dirs_lt(100_000);

    println!("Part 1: {result_part1}");
    Ok(())
}
