use std::str::FromStr;

#[derive(EnumString, EnumIter, Debug, Eq, PartialOrd, PartialEq, Clone, Copy)]
pub enum HandChoice {
    #[strum(serialize = "A", serialize = "X")]
    Rock = 1,
    #[strum(serialize = "B", serialize = "Y")]
    Paper = 2,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors = 3,
}

#[derive(EnumString, Debug, Eq, PartialOrd, PartialEq, Clone, Copy)]
pub enum Outcome {
    #[strum(serialize = "X")]
    Lose = 0,
    #[strum(serialize = "Y")]
    Draw = 3,
    #[strum(serialize = "Z")]
    Win = 6,
}

pub fn get_guide_pairs(input: &str) -> impl Iterator<Item=(HandChoice, HandChoice)> + '_ {
    input
        .split("\n")
        .filter_map(decode_pair)
}

pub fn get_real_guide_pairs(input: &str) -> impl Iterator<Item=(HandChoice, Outcome)> + '_ {
    input
        .split("\n")
        .filter_map(decode_real_pair)
}

fn decode_pair(pair: &str) -> Option<(HandChoice, HandChoice)> {
    let mut choices = pair.split_whitespace();

    Some((HandChoice::from_str(choices.next()?).ok()?, HandChoice::from_str(choices.next()?).ok()?))
}

fn decode_real_pair(pair: &str) -> Option<(HandChoice, Outcome)> {
    let mut choices = pair.split_whitespace();

    Some((HandChoice::from_str(choices.next()?).ok()?, Outcome::from_str(choices.next()?).ok()?))
}