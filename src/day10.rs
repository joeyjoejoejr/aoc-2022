use std::error::Error;
use std::str::FromStr;

#[cfg(feature = "test_input")]
fn get_input() -> Result<String> {
    Ok("
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
        .trim()
        .to_string())
}

#[cfg(not(feature = "test_input"))]
fn get_input() -> Result<String> {
    use std::fs;

    Ok(fs::read_to_string("inputs/day10input.txt")?)
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum Instruction {
    Noop { ticks: i32 },
    Addx { ticks: i32, value: i32 },
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(' ');
        Ok(match parts.next() {
            Some("noop") => Self::Noop { ticks: 1 },
            Some("addx") => Self::Addx {
                ticks: 2,
                value: parts.next().ok_or("invalid addx")?.parse()?,
            },
            _ => Err("invalid instruction")?,
        })
    }
}

impl Instruction {
    fn tick(&mut self, x: &mut i32) -> bool {
        match self {
            Self::Noop { ticks } => {
                *ticks -= 1;
                if *ticks == 0 {
                    return true;
                }
            }
            Self::Addx { ticks, value } => {
                *ticks -= 1;
                if *ticks == 0 {
                    *x += *value;
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> Result<()> {
    let input = get_input()?;
    let mut instructions = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| l.parse::<Instruction>().ok());
    let mut instruction = instructions.next().ok_or("missing first instruction")?;
    let mut x: i32 = 1;
    let mut signal_strength = 0;

    for i in 1.. {
        if (i - 20) % 40 == 0 {
            signal_strength += i * x;
        }
        if instruction.tick(&mut x) {
            if let Some(instr) = instructions.next() {
                instruction = instr;
            } else {
                break;
            }
        }
    }

    println!("Part 1: {signal_strength}");
    Ok(())
}
