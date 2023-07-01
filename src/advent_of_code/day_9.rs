use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{AddAssign, Sub},
};

use itertools::Itertools;

pub const INPUT: &'static str = include_str!("../../data/year_2022__day_9");

#[derive(Debug)]
struct Head {
    position: Coordinates,
}

#[derive(Debug)]
struct Tail {
    position: Coordinates,
}

#[derive(Debug)]
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
            // overlapping
            (0, 0) => (0, 0),
            // touching up/left/down/right
            (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
            // touching diagonally
            (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
            // need to move up/left/down/right
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // need to move to the right diagonally
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move to the left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            // 🆕 need to move diagonally
            (-2, -2) => (-1, -1),
            (-2, 2) => (-1, 1),
            (2, -2) => (1, -1),
            (2, 2) => (1, 1),
            (x, y) => panic!("unhandled case: head - tail = {x:?}, {y:?}"),
        };

        self.tail.position += tail_new_pos.into();
        self.movements.insert(self.tail.position.clone());
    }

    fn move_head_n_steps(&mut self, diff: Coordinates, n_steps: usize) {
        for _ in 0..n_steps {
            self.move_head(diff)
        }
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

pub struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new(size: usize) -> Self {
        Self {
            knots: (0..size).map(|_| Knot::default()).collect_vec(),
        }
    }

    fn move_rope(&mut self, diff: Coordinates) {
        let mut diff = diff;
        for i in 0..(self.knots.len() - 1) {
            self.knots[i].move_head(diff);
            diff = self.knots[i].tail.position - self.knots[i + 1].head.position;
        }
        let tl = self.knots.len() - 1;
        self.knots[tl].move_head(diff);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::{Coordinates, Knot, Movement};
    use super::{Rope, INPUT};
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
            let n_steps = trace.next().unwrap().parse::<usize>().unwrap();
            let movement: Movement = first.into();
            bridge.move_head_n_steps(movement.into(), n_steps)
        }
        assert_eq!(bridge.get_total_positions(), n_moves);
    }

    #[test_case(INPUT)]
    fn test_p_1(input: &str) {
        let mut bridge = Knot::default();
        for line in input.lines() {
            let mut trace = line.split_whitespace();
            let first = trace.next().unwrap().chars().nth(0).unwrap();
            let second = trace.next().unwrap().parse::<usize>().unwrap();
            let movement: Movement = first.into();
            bridge.move_head_n_steps(movement.into(), second)
        }
        assert_eq!(bridge.get_total_positions(), 6090); //after submission
    }

    #[test_case(
        r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#; "test_p_2_sample"
    )]
    fn test_p_2_sample(input: &str) {
        let mut rope = Rope::new(10);
        for line in input.lines() {
            let mut trace = line.split_whitespace();
            let direction = trace.next().unwrap().chars().nth(0).unwrap();
            let n_steps = trace.next().unwrap().parse::<usize>().unwrap();
            let movement: Movement = direction.into();
            let co: Coordinates = movement.into();
            for _ in 0..n_steps {
                rope.move_rope(co.into());
            }
        }
        let l = rope.knots.len();
        assert_eq!(rope.knots[l - 2].get_total_positions(), 36);
    }

    #[test_case(INPUT; "test_p_2")]
    fn test_p_2(input: &str) {
        let mut rope = Rope::new(10);
        for line in input.lines() {
            let mut trace = line.split_whitespace();
            let direction = trace.next().unwrap().chars().nth(0).unwrap();
            let n_steps = trace.next().unwrap().parse::<usize>().unwrap();
            let movement: Movement = direction.into();
            let co: Coordinates = movement.into();
            for _ in 0..n_steps {
                rope.move_rope(co.into());
            }
        }
        let l = rope.knots.len();
        assert_eq!(rope.knots[l - 2].get_total_positions(), 2566);
    }
}
