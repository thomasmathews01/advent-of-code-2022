use std::cmp::{max, min};
use array2d::Array2D;
use itertools::Itertools;

mod input;
pub use input::DAY8_INPUT;

pub fn get_day8_part1_answer(input: &str) -> usize {
    let (rows, cols) = (input.lines().count(), input.lines().next().unwrap().len());

    let tree_grid = Array2D::from_iter_row_major(input.chars().filter_map(|c| c.to_digit(10)), rows, cols).unwrap();

    let mut count_visible = 0;
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        let height = tree_grid.get(row, col).unwrap();

        let visible_from_left = tree_grid.row_iter(row).unwrap().take(col).all(|tree| tree < &height);
        let visible_from_right = tree_grid.row_iter(row).unwrap().rev().take(cols - 1 - col).all(|tree| tree < &height);
        let visible_from_top = tree_grid.column_iter(col).unwrap().take(row).all(|tree| tree < &height);
        let visible_from_bottom = tree_grid.column_iter(col).unwrap().rev().take(rows - 1 - row).all(|tree| tree < &height);

        count_visible += (visible_from_left || visible_from_right || visible_from_top || visible_from_bottom) as usize;
    }

    count_visible
}

pub fn get_day8_part2_answer<const N: usize>(input: &str) -> usize {
    let (rows, cols) = (input.lines().count(), input.lines().next().unwrap().len());

    let tree_grid = input.lines()
        .map(|line| line.chars() .filter_map(|c| c.to_digit(10)).array_chunks::<N>().next().unwrap())
        .array_chunks::<N>()
        .next().unwrap();

    let mut max_score = 0;

    for (row, col) in (0..rows).cartesian_product(0..cols) {
        let height = tree_grid[row][col];

        // Search tree's between this one and the edge, find the idx of the first tree we can't see pass (+1 to convert to count), or if we can see past all, use the count to the edge
        let visible_to_left = tree_grid[row][0..col].iter().rev().position(|tree| tree >= &height).map(|x| x + 1).unwrap_or(col);
        let visible_to_right = tree_grid[row][(col+1)..].iter().position(|tree| tree >= &height).map(|x| x + 1).unwrap_or(cols - 1 - col);
        let visible_to_top = tree_grid[0..row].iter().rev().map(|row| row[col]).position(|tree| tree >= height).map(|x| x + 1).unwrap_or(row);
        let visible_to_bottom = tree_grid[(row+1)..].iter().map(|row| row[col]).position(|tree| tree >= height).map(|x| x + 1).unwrap_or(rows - 1 - row);

        max_score = max(max_score, visible_to_left * visible_to_right * visible_to_top * visible_to_bottom);
    }

    max_score
}

#[cfg(test)]
mod tests {
    use crate::day8::{get_day8_part1_answer, get_day8_part2_answer};

    #[test]
    fn test() {
        assert_eq!(21, get_day8_part1_answer("30373
25512
65332
33549
35390"))
    }

    #[test]
    fn test2() {
        assert_eq!(8, get_day8_part2_answer::<5>("30373
25512
65332
33549
35390"))
    }
}