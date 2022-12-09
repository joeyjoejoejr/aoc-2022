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
    let mut positions = vec![(0, 0); 10];
    let mut short_visited = HashSet::new();
    let mut long_visited = HashSet::new();

    short_visited.insert(positions[1]);
    long_visited.insert(positions[9]);

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
                "U" => positions[0].1 -= 1,
                "D" => positions[0].1 += 1,
                "R" => positions[0].0 += 1,
                "L" => positions[0].0 -= 1,
                _ => unreachable!(),
            }
            for i in 1..positions.len() {
                let first = positions[i - 1];
                let mut next = &mut positions[i];

                let x_distance: i32 = first.0 - next.0;
                let y_distance: i32 = first.1 - next.1;
                if x_distance.pow(2) + y_distance.pow(2) > 4 {
                    next.0 += x_distance / x_distance.abs();
                    next.1 += y_distance / y_distance.abs();
                } else {
                    next.0 += x_distance / 2;
                    next.1 += y_distance / 2;
                }
            }

            short_visited.insert(positions[1]);
            long_visited.insert(positions[9]);
        }
    }

    println!("Part 1: {}", short_visited.len());
    println!("Part 2: {}", long_visited.len());
    Ok(())
}
