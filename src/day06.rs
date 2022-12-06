use std::error::Error;
use std::fs;

fn get_input() -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string("inputs/day06input.txt")?)
}

fn find_distinct_n(n: usize, slice: &str) -> usize {
    let mut position = n;

    loop {
        let mut prev_slice = slice[(position - n)..position]
            .chars()
            .collect::<Vec<char>>();
        prev_slice.sort();
        prev_slice.dedup();
        if prev_slice.len() == n {
            break position;
        } else {
            position += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let result = find_distinct_n(4, &input);
    println!("Part 1: {result}");

    let result = find_distinct_n(14, &input);
    println!("Part 2: {result}");
    Ok(())
}
