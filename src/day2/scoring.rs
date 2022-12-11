use crate::day2::decoding::{HandChoice, Outcome};
use crate::day2::decoding::Outcome::{Draw, Lose, Win};
use crate::strum::IntoEnumIterator;

pub fn get_round_score(round: (HandChoice, HandChoice)) -> u32 {
    let (_, my_choice) = round;

    my_choice as u32 + get_round_result(round) as u32
}

pub fn get_real_round_score(round: (HandChoice, Outcome)) -> u32 {
    let (opponents_choice, outcome) = round;
    let my_choice = HandChoice::iter().find(|choice| get_round_result((opponents_choice, *choice)) == outcome).unwrap();

    my_choice as u32 + outcome as u32
}

fn get_round_result(round: (HandChoice, HandChoice)) -> Outcome {
    let (opponents_choice, my_choice) = round;

    if my_choice == winning_choice_against(opponents_choice) {
        Win
    } else if my_choice == opponents_choice {
        Draw
    } else {
        Lose
    }
}

fn winning_choice_against(choice: HandChoice) -> HandChoice {
    match choice {
        HandChoice::Rock => HandChoice::Paper,
        HandChoice::Paper => HandChoice::Scissors,
        HandChoice::Scissors => HandChoice::Rock
    }
}