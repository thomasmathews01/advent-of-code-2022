use itertools::Itertools;

pub use input::DAY6_INPUT;

mod input;

pub fn get_day6_part1_answer(input: &str) -> usize {
    get_first_char_after_sequence_of_n_unique_chars(input, 4)
}

pub fn get_day6_part2_answer(input: &str) -> usize {
    get_first_char_after_sequence_of_n_unique_chars(input, 14)
}

fn get_first_char_after_sequence_of_n_unique_chars(input: &str, n: usize) -> usize {
    input.as_bytes().windows(n).position(|w| w.iter().all_unique()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day6::get_day6_part1_answer;

    #[test]
    fn test_works() {
        assert_eq!(5, get_day6_part1_answer("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, get_day6_part1_answer("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(11, get_day6_part1_answer("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}