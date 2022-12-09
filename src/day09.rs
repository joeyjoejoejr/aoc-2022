use std::collections::HashSet;
use std::error::Error;

#[cfg(feature = "test_input")]
fn get_input() -> Result<String> {
    Ok("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    .to_string())
}

#[cfg(not(feature = "test_input"))]
fn get_input() -> Result<String> {
    use std::fs;

    Ok(fs::read_to_string("inputs/day09input.txt")?)
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut visited_positions = HashSet::new();
    visited_positions.insert(tail_position);

    let moves = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| match l.split(' ').collect::<Vec<&str>>()[..] {
            [direction, count] => Some((direction, count.parse::<i32>().ok()?)),
            _ => None,
        })
        .collect::<Vec<(&str, i32)>>();

    for head_move in moves {
        for _ in 0..head_move.1 {
            match head_move.0 {
                "U" => head_position.1 -= 1,
                "D" => head_position.1 += 1,
                "R" => head_position.0 += 1,
                "L" => head_position.0 -= 1,
                _ => unreachable!(),
            }

            let x_distance: i32 = head_position.0 - tail_position.0;
            let y_distance: i32 = head_position.1 - tail_position.1;
            if x_distance.pow(2) + y_distance.pow(2) > 4 {
                tail_position.0 += x_distance / x_distance.abs();
                tail_position.1 += y_distance / y_distance.abs();
            } else {
                tail_position.0 += x_distance / 2;
                tail_position.1 += y_distance / 2;
            }
            visited_positions.insert(tail_position);
        }
    }
    println!("Part 1: {}", visited_positions.len());
    Ok(())
}
