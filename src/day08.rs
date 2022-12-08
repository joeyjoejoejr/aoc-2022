use std::error::Error;

#[cfg(feature = "test_input")]
fn get_input() -> Result<String> {
    Ok("30373
25512
65332
33549
35390"
        .to_string())
}

#[cfg(not(feature = "test_input"))]
fn get_input() -> Result<String> {
    use std::fs;

    Ok(fs::read_to_string("inputs/day08input.txt")?)
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let grid: Vec<Vec<u32>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect();
    let height = grid.len();
    let width = grid.first().ok_or("invalid grid")?.len();
    let mut visible = height * 2 + width * 2 - 4;
    let mut scenic_score = 0;

    for row in 1..height - 1 {
        for column in 1..width - 1 {
            let tree_height = grid[row][column];
            let mut tree_column = grid.iter().map(|row| row[column]);
            let top_max = tree_column.by_ref().take(row).max().ok_or("max failed")?;
            let bottom_max = tree_column.skip(1).max().ok_or("max failed")?;
            let left_max = grid[row][0..column].iter().max().ok_or("max failed")?;
            let right_max = grid[row][column + 1..].iter().max().ok_or("max failed")?;
            if tree_height > top_max
                || tree_height > bottom_max
                || tree_height > *left_max
                || tree_height > *right_max
            {
                visible += 1;
            }

            let tree_column = grid.iter().map(|row| row[column]);
            let top_score = tree_column
                .clone()
                .take(row)
                .skip(1)
                .rev()
                .take_while(|&t| t < tree_height)
                .count()
                + 1;
            let bottom_score = tree_column
                .skip(row + 1)
                .rev()
                .skip(1)
                .rev()
                .take_while(|&t| t < tree_height)
                .count()
                + 1;
            let left_score = grid[row][0..column]
                .iter()
                .skip(1)
                .rev()
                .take_while(|&&t| t < tree_height)
                .count()
                + 1;
            let right_score = grid[row][column + 1..]
                .iter()
                .rev()
                .skip(1)
                .rev()
                .take_while(|&&t| t < tree_height)
                .count()
                + 1;
            let total_score = top_score * bottom_score * left_score * right_score;
            if total_score > scenic_score {
                scenic_score = total_score;
            }
        }
    }

    println!("Part 1: {visible}");
    println!("Part 1: {scenic_score}");
    Ok(())
}
