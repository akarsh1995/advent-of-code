use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{AddAssign, Sub},
};

pub const INPUT: &'static str = include_str!("../data/year_2022__day_9");

struct Head {
    position: Coordinates,
}

struct Tail {
    position: Coordinates,
}

pub struct Knot {
    head: Head,
    tail: Tail,
    movements: HashSet<Coordinates>,
}

impl Default for Knot {
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

impl Knot {
    fn move_head(&mut self, diff: Coordinates) {
        self.head.position += diff.clone();
        let tail_new_pos = match (self.head.position - self.tail.position).into() {
            (0, 0) => (0, 0),
            // touching side by side
            // up | down | right | left
            (0, 1) | (0, -1) | (1, 0) | (-1, 0) => (0, 0),
            // touching diagonally
            // top-right | top-left | bottom-right | bottom-left
            (1, 1) | (-1, 1) | (1, -1) | (-1, -1) => (0, 0),
            // need to move up | down | right | left
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // need to move diagonal
            // top-right | top-left | bottom-right | bottom-left
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            (x, y) => panic!(
                "({x}, {y}) are not valid differences. {:?}, {:?}",
                self.head.position, self.tail.position
            ),
        };

        self.tail.position += tail_new_pos.into();
        self.movements.insert(self.tail.position.clone());
    }

    fn get_total_positions(&self) -> u32 {
        self.movements.len() as u32
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Copy)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, behind: Self) -> Self::Output {
        Self {
            x: self.x - behind.x,
            y: self.y - behind.y,
        }
    }
}

impl Into<(i32, i32)> for Coordinates {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl Into<Coordinates> for Movement {
    fn into(self) -> Coordinates {
        match self {
            Movement::Right => (1, 0),
            Movement::Left => (-1, 0),
            Movement::Up => (0, 1),
            Movement::Down => (0, -1),
        }
        .into()
    }
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
    use super::{Knot, Movement};
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
        let mut bridge = Knot::default();
        for line in input.lines() {
            let mut trace = line.split_whitespace();
            let first = trace.next().unwrap().chars().nth(0).unwrap();
            let second = trace.next().unwrap().parse::<u32>().unwrap();
            for _ in 0..second {
                let movement: Movement = first.into();
                bridge.move_head(movement.into())
            }
        }
        assert_eq!(bridge.get_total_positions(), n_moves);
    }

    #[test_case(INPUT)]
    fn test_p_1(input: &str) {
        let mut bridge = Knot::default();
        for line in input.lines() {
            let mut trace = line.split_whitespace();
            let first = trace.next().unwrap().chars().nth(0).unwrap();
            let second = trace.next().unwrap().parse::<u32>().unwrap();
            for _ in 0..second {
                let movement: Movement = first.into();
                bridge.move_head(movement.into())
            }
        }
        assert_eq!(bridge.get_total_positions(), 6090); //after submission
    }
}
