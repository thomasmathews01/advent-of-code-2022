#![feature(binary_heap_into_iter_sorted)]
#![feature(iterator_try_collect)]
extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::day1::{DAY1_INPUT, find_calories_carried_by_elf_with_most_calories, find_calories_carried_by_the_n_elves_with_most_calories};
use crate::day2::{actual_score, best_case_score, DAY2_INPUT};

mod day1;
mod day2;

fn main() {
    println!("Day 1 Part 1 Answer: {:?} calories carried by elf with most calories", find_calories_carried_by_elf_with_most_calories(DAY1_INPUT));
    println!("Day 1 Part 2 Answer: {:?} calories carried by the three elves with the most calories", find_calories_carried_by_the_n_elves_with_most_calories(DAY1_INPUT, 3));

    println!("Day 2 Part 1 Answer: {:?} (score if all games happen as planned)", best_case_score(DAY2_INPUT));
    println!("Day 2 Part 2 Answer: {:?} (score if all games happen as planned with correct interpretation of input)", actual_score(DAY2_INPUT));
}
