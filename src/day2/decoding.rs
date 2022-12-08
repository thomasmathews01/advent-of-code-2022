use std::str::FromStr;

#[derive(EnumString, EnumIter, Debug, Eq, PartialOrd, PartialEq, Clone, Copy)]
pub enum RpsChoice {
    #[strum(serialize="A", serialize="X")]
    Rock,
    #[strum(serialize="B", serialize="Y")]
    Paper,
    #[strum(serialize="C", serialize="Z")]
    Scissors
}

#[derive(EnumString, Debug, Eq, PartialOrd, PartialEq, Clone, Copy)]
pub enum Outcome {
    #[strum(serialize="X")]
    Lose,
    #[strum(serialize="Y")]
    Draw,
    #[strum(serialize="Z")]
    Win
}

pub fn get_guide_pairs(input: &str) -> impl Iterator<Item=(RpsChoice, RpsChoice)> + '_ {
    input
        .split("\n")
        .filter_map(decode_pair)
}

pub fn get_real_guide_pairs(input: &str) -> impl Iterator<Item=(RpsChoice, Outcome)> + '_ {
    input
        .split("\n")
        .filter_map(decode_real_pair)
}

fn decode_pair(pair: &str) -> Option<(RpsChoice, RpsChoice)> {
    let mut choices = pair.split_whitespace();

    Some((RpsChoice::from_str(choices.next()?).ok()?, RpsChoice::from_str(choices.next()?).ok()?))
}

fn decode_real_pair(pair: &str) -> Option<(RpsChoice, Outcome)> {
    let mut choices = pair.split_whitespace();

    Some((RpsChoice::from_str(choices.next()?).ok()?, Outcome::from_str(choices.next()?).ok()?))
}