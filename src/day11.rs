use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "test_input")]
fn get_input() -> Result<String> {
    Ok("
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
    .trim()
    .to_string())
}

#[cfg(not(feature = "test_input"))]
fn get_input() -> Result<String> {
    use std::fs;

    Ok(fs::read_to_string("inputs/day11input.txt")?)
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Monkey {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: usize,
    if_true: usize,
    if_false: usize,
    items_inspected: usize,
}

impl Monkey {
    fn inspect_next(&mut self, lcd: Option<usize>) -> Option<(usize, usize)> {
        let item = self.items.pop_front()?;
        self.items_inspected += 1;
        let worry_level = if let Some(lcd) = lcd {
            (self.operation)(item) % lcd
        } else {
            (self.operation)(item) / 3
        };

        if worry_level % self.test == 0 {
            Some((self.if_true, worry_level))
        } else {
            Some((self.if_false, worry_level))
        }
    }

    fn toss(&mut self, item: usize) {
        self.items.push_back(item)
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let items: VecDeque<usize> = lines
            .nth(1)
            .and_then(|l| l.split(':').nth(1))
            .ok_or("invalid monkey")?
            .split(", ")
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();

        let operation_strs: Vec<&str> = lines
            .next()
            .and_then(|l| l.split(':').nth(1))
            .ok_or("invalid_monkey")?
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();

        let operation = match operation_strs[..] {
            [_, _, "old", "+", "old"] => Box::new(|old| old + old) as Box<dyn Fn(usize) -> usize>,
            [_, _, "old", "+", value] => {
                let value = value.parse::<usize>()?;
                Box::new(move |old| old + value)
            }
            [_, _, "old", "*", "old"] => Box::new(|old| old * old),
            [_, _, "old", "*", value] => {
                let value = value.parse::<usize>()?;
                Box::new(move |old| old * value)
            }
            _ => Err("invalid operation")?,
        };

        let test = lines
            .next()
            .and_then(|l| l.split(' ').rev().next())
            .ok_or("invalid monkey")?
            .parse()?;
        let if_true = lines
            .next()
            .and_then(|l| l.split(' ').rev().next())
            .ok_or("invalid monkey")?
            .parse()?;
        let if_false = lines
            .next()
            .and_then(|l| l.split(' ').rev().next())
            .ok_or("invalid monkey")?
            .parse()?;

        Ok(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            items_inspected: 0,
        })
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("test", &self.test)
            .field("if_true", &self.if_true)
            .field("if_false", &self.if_false)
            .field("items_inspected", &self.items_inspected)
            .finish()
    }
}

fn main() -> Result<()> {
    let input = get_input()?;
    let mut monkeys = input
        .split("\n\n")
        .filter(|m| !m.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<Monkey>>>()?;

    for _i in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((idx, item)) = monkeys[i].inspect_next(None) {
                monkeys[idx].toss(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m1.items_inspected.cmp(&m2.items_inspected));
    monkeys.reverse();
    let monkey_business: usize = monkeys[0..2].iter().map(|m| m.items_inspected).product();

    println!("Part 1: {monkey_business}");

    let mut monkeys = input
        .split("\n\n")
        .filter(|m| !m.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<Monkey>>>()?;

    let lcd = monkeys.iter().map(|m| m.test).product();
    for _i in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some((idx, item)) = monkeys[i].inspect_next(Some(lcd)) {
                monkeys[idx].toss(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m1.items_inspected.cmp(&m2.items_inspected));
    monkeys.reverse();
    let monkey_business: usize = monkeys[0..2].iter().map(|m| m.items_inspected).product();

    println!("Part 2: {monkey_business}");

    Ok(())
}
