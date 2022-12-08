use crate::day2::decoding::{Outcome, RpsChoice};
use crate::strum::IntoEnumIterator;

pub fn get_round_score(round: (RpsChoice, RpsChoice)) -> u32 {
    let (_, my_choice) = round;

    get_shape_score(my_choice) + get_victory_score(get_round_result(round))
}

pub fn get_real_round_score(round: (RpsChoice, Outcome)) -> u32 {
    let (opponents_choice, outcome) = round;
    let my_choice = RpsChoice::iter().find(|choice| get_round_result((opponents_choice, *choice)) == outcome).unwrap();

    get_shape_score(my_choice) + get_victory_score(outcome)
}

fn get_shape_score(choice: RpsChoice) -> u32 {
    match choice {
        RpsChoice::Rock => 1,
        RpsChoice::Paper => 2,
        RpsChoice::Scissors => 3
    }
}

fn get_victory_score(result: Outcome) -> u32 {
    match result {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    }
}

fn get_round_result(round: (RpsChoice, RpsChoice)) -> Outcome {
    let (opponents_choice, my_choice) = round;

    if my_choice == winning_choice_against(opponents_choice) {
        Outcome::Win
    } else if my_choice == opponents_choice {
        Outcome::Draw
    } else {
        Outcome::Lose
    }
}

fn winning_choice_against(choice: RpsChoice) -> RpsChoice {
    match choice {
        RpsChoice::Rock => RpsChoice::Paper,
        RpsChoice::Paper => RpsChoice::Scissors,
        RpsChoice::Scissors => RpsChoice::Rock
    }
}