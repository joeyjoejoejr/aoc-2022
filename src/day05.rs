use std::convert::TryInto;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Move {
    count: usize,
    source: usize,
    destination: usize,
}

impl Move {
    fn apply(&self, stacks: &mut [Vec<char>]) -> Result<(), Box<dyn Error>> {
        for _ in 0..self.count {
            let value = stacks[self.source - 1].pop().ok_or("failed to pop")?;
            stacks[self.destination - 1].push(value);
        }
        Ok(())
    }

    fn apply9001(&self, stacks: &mut [Vec<char>]) -> Result<(), Box<dyn Error>> {
        let source = stacks[self.source - 1].clone();
        let source_len = source.len();
        stacks[self.destination - 1].extend(source.iter().rev().take(self.count).rev());

        stacks[self.source - 1].truncate(source_len - self.count);

        Ok(())
    }
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_line = s.split(' ');
        let count = split_line.nth(1).ok_or("Invalid input -- count")?.parse()?;
        let source = split_line
            .nth(1)
            .ok_or("Invalid input -- source")?
            .parse()?;
        let destination = split_line
            .nth(1)
            .ok_or("Invalid input -- destination")?
            .parse()?;

        Ok(Move {
            count,
            source,
            destination,
        })
    }
}

#[cfg(test_input)]
fn get_input() -> Result<String, Box<dyn Error>> {
    Ok("    [D]    \n\
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        .to_string())
}

#[cfg(not(test_input))]
fn get_input() -> Result<String, Box<dyn Error>> {
    use std::fs;

    Ok(fs::read_to_string("inputs/day05input.txt")?)
}

fn parse_stack_input(stack_input: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut lines = stack_input.lines().rev();
    let stack_count = lines.next().ok_or("Invalid input")?.len() / 4 + 1;
    let mut result = vec![vec![]; stack_count];

    for line in lines {
        for (i, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            match chunk {
                [_, label, _, _] if *label != ' ' => result[i].push(*label),
                [_, label, _] if *label != ' ' => result[i].push(*label),
                _ => (),
            }
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let [stack_input, move_input]: [&str; 2] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .map_err(|_| "Invalid input")?;

    let mut stacks = parse_stack_input(stack_input)?;
    let mut stacks2 = stacks.clone();
    let moves = move_input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<Move>, _>>()?;

    for mv in &moves {
        mv.apply(&mut stacks)?;
        mv.apply9001(&mut stacks2)?;
    }

    let (result, result2): (String, String) = stacks
        .iter()
        .zip(stacks2.iter())
        .filter_map(|(stack, stack2)| Some((stack.last()?, stack2.last()?)))
        .unzip();

    println!("Day 1: {result}");
    println!("Day 2: {result2}");
    Ok(())
}
