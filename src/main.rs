#![feature(binary_heap_into_iter_sorted)]
#![feature(iterator_try_collect)]
#![feature(iter_array_chunks)]
extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::day1::{DAY1_INPUT, find_calories_carried_by_elf_with_most_calories, find_calories_carried_by_the_n_elves_with_most_calories};
use crate::day2::{actual_score, best_case_score, DAY2_INPUT};
use crate::day3::{DAY3_INPUT, get_day3_part1_answer, get_day3_part2_answer};
use crate::day4::{DAY4_INPUT, get_day4_part1_answer, get_day4_part2_answer};
use crate::day5::{DAY5_INPUT, get_day5_part1_answer, get_day5_part2_answer};
use crate::day6::{DAY6_INPUT, get_day6_part1_answer, get_day6_part2_answer};
use crate::day7::{DAY7_INPUT, get_day7_part1_answer, get_day7_part2_answer};
use crate::day8::{DAY8_INPUT, get_day8_part1_answer, get_day8_part2_answer};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let complete_start = std::time::Instant::now();

    let day1_start = std::time::Instant::now();
    println!("Day 1 Part 1 Answer: {:?} calories carried by elf with most calories", find_calories_carried_by_elf_with_most_calories(DAY1_INPUT));
    println!("Day 1 Part 2 Answer: {:?} calories carried by the three elves with the most calories", find_calories_carried_by_the_n_elves_with_most_calories(DAY1_INPUT, 3));
    println!("Day 1 took {:?}", day1_start.elapsed());

    let day2_start = std::time::Instant::now();
    println!("Day 2 Part 1 Answer: {:?} (score if all games happen as planned)", best_case_score(DAY2_INPUT));
    println!("Day 2 Part 2 Answer: {:?} (score if all games happen as planned with correct interpretation of input)", actual_score(DAY2_INPUT));
    println!("Day 2 took {:?}", day2_start.elapsed());

    let day3_start = std::time::Instant::now();
    println!("Day 3 Part 1 Answer: {:?}", get_day3_part1_answer(DAY3_INPUT));
    println!("Day 3 Part 2 Answer: {:?}", get_day3_part2_answer(DAY3_INPUT));
    println!("Day 3 took {:?}", day3_start.elapsed());

    let day4_start = std::time::Instant::now();
    println!("Day 4 Part 1 Answer: {:?}", get_day4_part1_answer(DAY4_INPUT));
    println!("Day 4 Part 2 Answer: {:?}", get_day4_part2_answer(DAY4_INPUT));
    println!("Day 4 took {:?}", day4_start.elapsed());

    let day5_start = std::time::Instant::now();
    println!("Day 5 Part 1 Answer: {:?}", get_day5_part1_answer(DAY5_INPUT));
    println!("Day 5 Part 2 Answer: {:?}", get_day5_part2_answer(DAY5_INPUT));
    println!("Day 5 took {:?}", day5_start.elapsed());

    let day6_start = std::time::Instant::now();
    println!("Day 6 Part 1 Answer: {:?}", get_day6_part1_answer(DAY6_INPUT));
    println!("Day 6 Part 2 Answer: {:?}", get_day6_part2_answer(DAY6_INPUT));
    println!("Day 6 took {:?}", day6_start.elapsed());

    let day7_start = std::time::Instant::now();
    println!("Day 7 Part 1 Answer: {:?}", get_day7_part1_answer(DAY7_INPUT));
    println!("Day 7 Part 2 Answer: {:?}", get_day7_part2_answer(DAY7_INPUT));
    println!("Day 7 took {:?}", day7_start.elapsed());

    let day8_start = std::time::Instant::now();
    println!("Day 8 Part 1 Answer: {:?}", get_day8_part1_answer(DAY8_INPUT));
    println!("Day 8 Part 2 Answer: {:?}", get_day8_part2_answer::<99>(DAY8_INPUT));
    println!("Day 8 took {:?}", day8_start.elapsed());

    println!("Whole thing took: {:?}", complete_start.elapsed())
}
