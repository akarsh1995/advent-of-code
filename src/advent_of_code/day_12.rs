// use itertools::Itertools;
// const SAMPLE_INPUT: &'static str = r#"Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi"#;

// #[derive(Debug, PartialEq, Eq)]
// struct GridPos {
//     x: i32,
//     y: i32,
// }

// impl GridPos {
//     fn up(&self) -> Self {
//         Self {
//             x: self.x,
//             y: self.y + 1,
//         }
//     }

//     fn down(&self) -> Self {
//         Self {
//             x: self.x,
//             y: self.y - 1,
//         }
//     }

//     fn left(&self) -> Self {
//         Self {
//             x: self.x - 1,
//             y: self.y,
//         }
//     }

//     fn right(&self) -> Self {
//         Self {
//             x: self.x + 1,
//             y: self.y,
//         }
//     }
// }

// impl Default for GridPos {
//     fn default() -> Self {
//         Self { x: 0, y: 0 }
//     }
// }

// impl Into<(i32, i32)> for GridPos {
//     fn into(self) -> (i32, i32) {
//         (self.x, self.y)
//     }
// }

// #[derive(Debug, PartialEq, Eq)]
// struct HillClimb {
//     grid: Vec<Vec<char>>,
//     current_pos: GridPos,
//     step_track: Vec<Vec<char>>,
// }

// impl HillClimb {
//     fn parse(raw_input: &str) -> Self {
//         let grid = raw_input
//             .lines()
//             .map(|l| l.chars().collect_vec())
//             .collect::<Vec<Vec<char>>>();
//         let step_track = raw_input
//             .lines()
//             .map(|l| l.chars().map(|_| '.').collect_vec())
//             .collect::<Vec<Vec<char>>>();
//         Self {
//             grid,
//             current_pos: GridPos::default(),
//             step_track,
//         }
//     }

//     fn get_value(&self, grid_pos: &GridPos) -> u32 {
//         self.grid[grid_pos.x as usize][-grid_pos.y as usize] as u32
//     }

//     fn step(&mut self) {
//         let up = self.current_pos.up();
//         let down = self.current_pos.down();
//         let left = self.current_pos.left();
//         let right = self.current_pos.right();

//         if self.check_pos_and_replace(up) {return;};
//         if self.check_pos_and_replace(down) {return;};
//         if self.check_pos_and_replace(left) { return;};
//         if self.check_pos_and_replace(right) {return;};

//     }

//     fn check_pos_and_replace(&mut self, pos: GridPos) -> bool {
//         let current_val = self.get_value(&self.current_pos);
//         if !self.out_of_grid(&pos) {
//             let val = self.get_value(&pos);
//             if current_val + 1 == val || current_val == 'S' as u32 {
//                 println!("valid position {:?}", &pos);
//                 self.current_pos = pos;
//                 return true;
//             }
//         }
//         false
//     }

//     fn out_of_grid(&self, other: &GridPos) -> bool {
//         other.x < 0
//         || other.x > (self.grid[0].len() - 1) as i32
//         || other.y > 0
//         || other.y < -1 * (self.grid.len() - 1) as i32
//     }
// }

// enum Direction {
//     Down,
//     Up,
//     Left,
//     Right,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test_case::test_case;
//     #[test_case( SAMPLE_INPUT,
//         HillClimb {
//             grid: vec![
//                     vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm', ],
//                     vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l', ],
//                     vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k', ],
//                     vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j', ],
//                     vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i', ],
//             ],
//             current_pos: GridPos { x: 0, y: 0},
//             step_track: vec![vec![]]

//         }; "test_parsing_input"
//     )]
//     fn test_parsing_input(input: &str, out: HillClimb) {
//         assert_eq!(HillClimb::parse(input), out);
//     }

//     #[test_case( SAMPLE_INPUT; "test_movement")]
//     fn test_movement(input: &str) {
//         let mut hc =         HillClimb::parse(input);
//         for _ in 0..10 {
//             hc.step();
//         }
//     }
// }
