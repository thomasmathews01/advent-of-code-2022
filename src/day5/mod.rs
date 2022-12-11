use tinyvec::ArrayVec;

pub use input::DAY5_INPUT;

type Stacks = [ArrayVec<[char; 50]>; 9];

mod input;

struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}

pub fn get_day5_part1_answer(input: &str) -> String {
    carryout_simulation::<CrateMover9000>(input)
}

pub fn get_day5_part2_answer(input: &str) -> String {
    carryout_simulation::<CrateMover9001>(input)
}

fn carryout_simulation<C: Crane>(input: &str) -> String {
    let mut stacks = parse_initial_stacks(input);

    for instruction in parse_instructions(input) {
        C::carryout_instruction(&mut stacks, instruction)
    }

    stacks.iter_mut().filter_map(ArrayVec::pop).collect()
}

fn parse_initial_stacks(input: &str) -> Stacks {
    let mut stacks: Stacks = Default::default();

    for line in input.lines().take_while(|line| line.contains("[")) {
        for (char_idx, crate_char) in line.chars().enumerate().filter(|(_idx, c)| c.is_alphanumeric()) {
            stacks[(char_idx - 1) / 4].push(crate_char);
        }
    }

    stacks.iter_mut().for_each(|s| s.reverse());

    stacks
}

fn parse_instructions(input: &str) -> impl Iterator<Item=Instruction> + '_ {
    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .filter_map(parse_instruction)
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let mut numbers = line
        .split_whitespace()
        .map(str::parse)
        .filter_map(Result::ok);

    Some(Instruction {
        count: numbers.next()?,
        src: numbers.next()? - 1, // Convert into index
        dst: numbers.next()? - 1,
    })
}

trait Crane {
    fn carryout_instruction(stacks: &mut Stacks, instruction: Instruction);
}

struct CrateMover9000;

impl Crane for CrateMover9000 {
    fn carryout_instruction(stacks: &mut Stacks, instruction: Instruction) {
        for _ in 0..instruction.count {
            let cr = stacks[instruction.src].pop().unwrap();
            stacks[instruction.dst].push(cr);
        }
    }
}

struct CrateMover9001;

impl Crane for CrateMover9001 {
    fn carryout_instruction(stacks: &mut Stacks, instruction: Instruction) {
        let moved_crates = stacks[instruction.src].split_off(stacks[instruction.src].len() - instruction.count);
        stacks[instruction.dst].extend_from_slice(&moved_crates)
    }
}