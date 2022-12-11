pub use input::DAY2_INPUT;

use crate::day2::decoding::{get_guide_pairs, get_real_guide_pairs};
use crate::day2::scoring::{get_real_round_score, get_round_score};

mod input;
mod decoding;
mod scoring;

pub fn best_case_score(input: &str) -> u32 {
    get_guide_pairs(input)
        .map(get_round_score)
        .sum()
}

pub fn actual_score(input: &str) -> u32 {
    get_real_guide_pairs(input)
        .map(get_real_round_score)
        .sum()
}