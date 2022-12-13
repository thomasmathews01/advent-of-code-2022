mod input;

use itertools::Itertools;
use crate::strum::IntoEnumIterator;
pub use input::DAY9_INPUT;

#[derive(Default, PartialOrd, PartialEq, Copy, Clone, Debug, Ord, Eq)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Default, PartialOrd, PartialEq, Copy, Clone, Debug)]
struct State {
    head_loc: Coords,
    tail_loc: Coords,
}

#[derive(EnumIter, PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum MovementType {
    Up,
    Down,
    Left,
    Right,
}

pub fn get_day9_part1_answer(input: &str) -> usize {
    parse_instructions(input)
        .scan(State::default(), perform_movement)
        .map(|state| state.tail_loc)
        .sorted()
        .dedup()
        .count()
}

pub fn parse_instructions(input: &str) -> impl Iterator<Item=MovementType> + '_ {
    input.lines().map(|line|
        std::iter::repeat(match &line.trim()[0..1] {
            "L" => MovementType::Left,
            "R" => MovementType::Right,
            "U" => MovementType::Up,
            _ => MovementType::Down
        }).take(line.trim()[2..].parse().unwrap())
    ).flatten()
}

fn move_in_direction(coords: Coords, movement: MovementType) -> Coords {
    match movement {
        MovementType::Up => Coords { x: coords.x, y: coords.y + 1 },
        MovementType::Down => Coords { x: coords.x, y: coords.y - 1 },
        MovementType::Left => Coords { x: coords.x - 1, y: coords.y },
        MovementType::Right => Coords { x: coords.x + 1, y: coords.y },
    }
}

fn squared_distance(c1: &Coords, c2: &Coords) -> i32 {
    (c1.x - c2.x).pow(2) + (c1.y - c2.y).pow(2)
}

fn perform_movement(old_state: &mut State, movement: MovementType) -> Option<State> {
    let mut new_state = old_state.clone();

    new_state.head_loc = move_in_direction(new_state.head_loc, movement);
    let distance_from_head = |loc: &Coords | squared_distance(&new_state.head_loc, loc);
    let distance_from_prev_tail = |loc: &Coords | squared_distance(&old_state.tail_loc, loc);

    if distance_from_head(&new_state.tail_loc) >= 4 {
        while squared_distance(&new_state.head_loc, &new_state.tail_loc) > 1 {
            new_state.tail_loc = MovementType::iter()
                .map(|movement| move_in_direction(new_state.tail_loc, movement))
                .min_by(|a, b|
                    distance_from_head(a).cmp(&distance_from_head(b))
                        .then(distance_from_prev_tail(a).cmp(&distance_from_prev_tail(b)))
                )?;
        }
    };

    *old_state = new_state;
    Some(new_state)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::day9::{Coords, get_day9_part1_answer, MovementType, parse_instructions, perform_movement, squared_distance, State};
    use crate::day9::MovementType::*;

    #[test]
    fn test_parsing_instructions() {
        assert_eq!(
            &[Left, Left, Down, Down, Up, Right],
            &parse_instructions("L 2\nD 2\nU 1\nR 1").collect_vec().as_slice());
    }

    #[test]
    fn test1() {
        assert_eq!(
            13,
            get_day9_part1_answer("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"));
    }

    #[test]
    fn test_movement_up() {
        let mut initial_state = State {
            tail_loc: Coords {x: 0, y: 0},
            head_loc: Coords {x: 0, y: 1},
        };

        let new_state = perform_movement(&mut initial_state, Up).unwrap();
        assert_eq!(new_state, State {
            tail_loc: Coords {x: 0, y: 1},
            head_loc: Coords {x: 0, y: 2},
        });
    }

    #[test]
    fn test_movement_down() {
        let mut initial_state = State {
            tail_loc: Coords {x: 0, y: 0},
            head_loc: Coords {x: 0, y: -1},
        };

        let new_state = perform_movement(&mut initial_state, Down).unwrap();
        assert_eq!(new_state, State {
            tail_loc: Coords {x: 0, y: -1},
            head_loc: Coords {x: 0, y: -2},
        });
    }

    #[test]
    fn test_distance() {
        assert_eq!(4, squared_distance(&Coords {x: 0, y: 1}, &Coords {x: 0, y: -1}))
    }

    #[test]
    fn test_movement_left() {
        let mut initial_state = State {
            tail_loc: Coords {x: 0, y: 0},
            head_loc: Coords {x: -1, y: 0},
        };

        let new_state = perform_movement(&mut initial_state, Left).unwrap();
        assert_eq!(new_state, State {
            tail_loc: Coords {x: -1, y: 0},
            head_loc: Coords {x: -2, y: 0},
        });
    }

    #[test]
    fn test_movement_right() {
        let mut initial_state = State {
            tail_loc: Coords {x: 0, y: 0},
            head_loc: Coords {x: 1, y: 0},
        };

        let new_state = perform_movement(&mut initial_state, Right).unwrap();
        assert_eq!(new_state, State {
            tail_loc: Coords {x: 1, y: 0},
            head_loc: Coords {x: 2, y: 0},
        });
    }

    #[test]
    fn test_movement_diagonal() {
        let mut initial_state = State {
            tail_loc: Coords {x: 0, y: 0},
            head_loc: Coords {x: 1, y: 1},
        };

        let mut new_state = perform_movement(&mut initial_state, Right).unwrap();
        let next_new_state = perform_movement(&mut new_state, Up).unwrap();
        assert_eq!(next_new_state, State {
            tail_loc: Coords {x: 1, y: 1},
            head_loc: Coords {x: 2, y: 2},
        });
    }

    #[test]
    fn test_movement_diagonal2() {
        let mut initial_state = State {
            tail_loc: Coords {x: 0, y: 0},
            head_loc: Coords {x: -1, y: 1},
        };

        let mut new_state = perform_movement(&mut initial_state, Left).unwrap();
        let next_new_state = perform_movement(&mut new_state, Up).unwrap();
        assert_eq!(next_new_state, State {
            tail_loc: Coords {x: -1, y: 1},
            head_loc: Coords {x: -2, y: 2},
        });
    }
}