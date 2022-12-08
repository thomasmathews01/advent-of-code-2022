use std::collections::{BinaryHeap};
pub use crate::day1::input::DAY1_INPUT;

mod input;

pub fn find_calories_carried_by_elf_with_most_calories(input: &str) -> u32 {
    let meals_grouped_by_elf = parse_into_groups(input);

    meals_grouped_by_elf
        .map(Iterator::sum::<u32>) // Total calories per elf
        .max()
        .unwrap() // We assume there is at least one element, because one can see there is in the input... Is this cheating?
}

pub fn find_calories_carried_by_the_n_elves_with_most_calories(input: &str, elf_count: usize) -> u32 {
    let meals_grouped_by_elf = parse_into_groups(input);
    let calorie_counts: BinaryHeap<u32> = meals_grouped_by_elf.map(Iterator::sum::<u32>).collect();

    calorie_counts.into_iter_sorted().take(elf_count).sum()
}

fn parse_into_groups<'a>(raw_input: &'a str) -> impl Iterator<Item=impl Iterator<Item=u32> + 'a> + 'a {
    let elf_splices = raw_input
        .split("\n\n");

     elf_splices.map(|chunk| chunk.split("\n").filter_map(|num| num.parse().ok()))
}

#[cfg(test)]
mod tests {
    use crate::day1::{find_calories_carried_by_elf_with_most_calories, parse_into_groups};

    #[test]
    fn can_split_into_groups() {
        let input =
            "123
234

345
456

567
678
789
";
        let groups = parse_into_groups(input).map(Iterator::collect::<Vec<_>>).collect::<Vec<_>>();
        assert_eq!(3, groups.len());

        assert_eq!(vec![123, 234], groups[0]);
        assert_eq!(vec![345, 456], groups[1]);
        assert_eq!(vec![567, 678, 789], groups[2]);
    }

    #[test]
    fn can_get_largest_group() {
        let input =
"123
234

345
456

567
678
789
";
        let calories_carried = find_calories_carried_by_elf_with_most_calories(input);
        assert_eq!(567+678+789, calories_carried);
    }
}