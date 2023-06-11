use std::{collections::HashSet, fmt::Debug, ptr::write};

pub const INPUT: &'static str = include_str!("../data/year_2022__day_9");

struct Head {
    position: Coordinates,
}

struct Tail {
    position: Coordinates,
}

pub struct Bridge {
    head: Head,
    tail: Tail,
    movements: HashSet<Coordinates>,
}

impl Default for Bridge {
    fn default() -> Self {
        let mut h = HashSet::new();
        h.insert((0, 0).into());
        Self {
            head: Head {
                position: Coordinates { x: 0, y: 0 },
            },
            tail: Tail {
                position: Coordinates { x: 0, y: 0 },
            },
            movements: h,
        }
    }
}

impl Bridge {
    fn move_head(&mut self, to: Movement) {
        // if heads and tails are on the same coordinates then only move heads not tails
        let old_pos = self.head.position.clone();
        self.head.position = self.head.position.move_to(&to);
        let new_pos = old_pos.move_to(&to);

        if new_pos.is_diagonal(&self.tail.position)
            || new_pos.overlaps(&self.tail.position)
            || new_pos.is_beside(&self.tail.position)
        {
            return;
        }

        self.tail.position = old_pos.clone();
        self.movements.insert(self.tail.position.clone());
    }

    fn get_total_positions(&self) -> u32 {
        self.movements.len() as u32
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinates {
    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn move_to(&self, m: &Movement) -> Self {
        match m {
            Movement::Right => (self.x + 1, self.y),
            Movement::Left => (self.x - 1, self.y),
            Movement::Up => (self.x, self.y + 1),
            Movement::Down => (self.x, self.y - 1),
        }
        .into()
    }

    fn is_diagonal(&self, other: &Coordinates) -> bool {
        (*other == self.move_to(&Movement::Right).move_to(&Movement::Up))
            || *other == self.move_to(&Movement::Right).move_to(&Movement::Down)
            || *other == self.move_to(&Movement::Left).move_to(&Movement::Down)
            || *other == self.move_to(&Movement::Left).move_to(&Movement::Up)
    }

    fn overlaps(&self, other: &Coordinates) -> bool {
        self == other
    }

    fn is_beside(&self, other: &Coordinates) -> bool {
        *other == self.move_to(&Movement::Right)
            || *other == self.move_to(&Movement::Left)
            || *other == self.move_to(&Movement::Up)
            || *other == self.move_to(&Movement::Down)
    }
}

enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => panic!("Wrong Movement"),
        }
    }
}

impl From<(i32, i32)> for Coordinates {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::INPUT;
    use super::{Bridge, Coordinates, Movement};
    use test_case::test_case;

    #[test_case(
        r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#,
        13
    )]
    fn test_a(input: &str, n_moves: u32) {
        let mut bridge = Bridge::default();
        for line in input.lines() {
            let mut trace = line.split_whitespace();
            let first = trace.next().unwrap().chars().nth(0).unwrap();
            let second = trace.next().unwrap().parse::<u32>().unwrap();
            for _ in 0..second {
                let movement: Movement = first.into();
                bridge.move_head(movement)
            }
        }
        assert_eq!(bridge.get_total_positions(), n_moves);
    }

    #[test_case(INPUT)]
    fn test_p_1(input: &str) {
        let mut bridge = Bridge::default();
        for line in input.lines() {
            let mut trace = line.split_whitespace();
            let first = trace.next().unwrap().chars().nth(0).unwrap();
            let second = trace.next().unwrap().parse::<u32>().unwrap();
            for _ in 0..second {
                let movement: Movement = first.into();
                bridge.move_head(movement)
            }
        }
        dbg!(bridge.get_total_positions());
    }
}
