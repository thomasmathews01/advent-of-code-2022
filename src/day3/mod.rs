use tinyset::Set64;

pub use input::DAY3_INPUT;

mod input;

pub fn get_day3_part1_answer(input: &str) -> u32 {
    input.lines().filter_map(get_value_of_line).sum()
}

fn get_value_of_line(line: &str) -> Option<u32> {
    let compartment1: Set64<char> = line.chars().take(line.len() / 2).collect();
    let compartment2: Set64<char> = line.chars().skip(line.len() / 2).collect();

    Some(get_priority_value(intersection([compartment1, compartment2])?))
}

pub fn get_day3_part2_answer(input: &str) -> u32 {
    input.lines()
        .map(str::chars)
        .map(Iterator::collect::<Set64<_>>)
        .array_chunks::<3>()
        .filter_map(intersection)
        .map(get_priority_value)
        .sum()
}

fn intersection<const N: usize>(group: [Set64<char>; N]) -> Option<char> {
    group.first()?.iter().filter(|c| group.iter().all(|s| s.contains(c))).next()
}

fn get_priority_value(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 'a' as u32 + 1,
        false => c as u32 - 'A' as u32 + 1 + 26
    }
}