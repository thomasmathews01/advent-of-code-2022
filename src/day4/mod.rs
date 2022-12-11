pub use input::DAY4_INPUT;

mod input;

pub fn get_day4_part1_answer(input: &'static str) -> usize {
    input
        .lines()
        .filter_map(get_pairs_from_line)
        .filter(subset)
        .count()
}

pub fn get_day4_part2_answer(input: &'static str) -> usize {
    input
        .lines()
        .filter_map(get_pairs_from_line)
        .filter(overlap)
        .count()
}

pub fn get_pairs_from_line(line: &'static str) -> Option<((u8, u8), (u8, u8))> {
    let [p1a, p1b, p2a, p2b] = line
        .split(&['-', ','])
        .filter_map(|num| num.parse().ok())
        .array_chunks::<4>().next()?;

    Some(((p1a, p1b), (p2a, p2b)))
}

pub fn subset(pairs: &((u8, u8), (u8, u8))) -> bool {
    let ((p1a, p1b), (p2a, p2b)) = pairs;

    p1a <= p2a && p1b >= p2b || p1a >= p2a && p1b <= p2b
}

pub fn overlap(pairs: &((u8, u8), (u8, u8))) -> bool {
    let ((p1a, p1b), (p2a, p2b)) = pairs;

    !(*p1a > *p2b || *p1b < *p2a)
}

#[cfg(test)]
mod tests {
    use crate::day4::overlap;

    #[test]
    fn test_overlap() {
        assert_eq!(false, overlap(&((2, 4), (6, 8))));
        assert_eq!(false, overlap(&((2, 3), (4, 5))));
        assert_eq!(true, overlap(&((5, 7), (7, 9))));
        assert_eq!(true, overlap(&((2, 8), (3, 7))));
        assert_eq!(true, overlap(&((6, 6), (4, 6))));
        assert_eq!(true, overlap(&((2, 6), (4, 8))));
    }
}