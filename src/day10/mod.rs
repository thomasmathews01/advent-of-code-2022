use std::iter::repeat;
use itertools::Itertools;
use crate::day10::Instruction::{Add, NoOp};

mod input;
pub use input::DAY10_INPUT;

enum Instruction {
    NoOp,
    Add(i32)
}

pub fn get_day10_part1_answer(input: &str, first: usize, step: usize) -> i32 {
    get_cycles(input)
        .enumerate()
        .map(|(cycle, register_val)| (cycle + 1) as i32 * register_val)
        .skip(first - 1)
        .step_by(step)
        .sum()
}

pub fn get_day10_part2_answer(input: &str) -> String {
    get_cycles(input)
        .enumerate()
        .map(|(cpu_cycle, register_val)| (cpu_cycle as i32 % 40).abs_diff(register_val) < 2)
        .map(|sprite_visible| if sprite_visible {'#'} else {'.'})
        .array_chunks::<40>()
        .map(|display_line| display_line.into_iter().collect::<String>())
        .join("\n")
}

fn get_cycles(input: &str) -> impl Iterator<Item=i32> + '_ {
    let instructions = parse_instructions(input);
    run_instructions(instructions)
}

fn parse_instructions(input: &str) -> impl Iterator<Item=Instruction>+ '_  {
    input.lines().map(|line| match line {
        "noop" => NoOp,
        add_instr => Add(add_instr.split_whitespace().map(str::parse).filter_map(Result::ok).next().unwrap())
    })
}

fn run_instructions(instructions: impl Iterator<Item=Instruction>) -> impl Iterator<Item=i32> {
    std::iter::from_generator(|| {
        let mut counter = 1;

        for instr in instructions {
            for cycle in generate_cycles(&mut counter, instr) {
                yield cycle
            }
        }
    })
}

fn generate_cycles(current_value: &mut i32, instruction: Instruction) -> impl Iterator<Item=i32> {
    match instruction {
        NoOp => repeat(*current_value).take(1),
        Add(count) => {
            let old_value = *current_value;
            *current_value += count;
            repeat(old_value).take(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::{get_day10_part1_answer, get_day10_part2_answer};

    #[test]
    fn test_input2() {
        let result = get_day10_part1_answer("noop
addx 3
addx -5",1, 1);

}

    #[test]
    fn test_input() {
        let result = get_day10_part2_answer("addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop");

        println!("{}", result)
    }
}