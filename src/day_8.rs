#[allow(unused)]
use std::{collections::HashSet, io::Read};

pub const INPUT: &'static str = include_str!("../data/year_2022__day_8");

trait RowCompare {
    fn calc(&self, row: &Vec<u32>) -> Vec<u32>;
}

impl RowCompare for Vec<u32> {
    fn calc(&self, row: &Vec<u32>) -> Vec<u32> {
        self.iter()
            .zip(row)
            .map(|(x, y)| y > x)
            .map(|f| f as u32)
            .collect::<Vec<u32>>()
    }
}

pub fn get_sum(input_matrix: &Vec<Vec<u32>>, pairs: &mut HashSet<(usize, usize)>) {
    let nrows = input_matrix.len();
    let ncols = input_matrix[0].len();

    for col in 0..ncols {
        let mut last_big = -1;
        for row in 0..nrows {
            if input_matrix[row][col] as i32 > last_big {
                pairs.insert((row, col));
                last_big = input_matrix[row][col] as i32;
            }
        }
    }

    for col in 0..ncols {
        let mut last_big = -1;
        for row in (0..=(nrows - 1)).rev() {
            if input_matrix[row][col] as i32 > last_big {
                pairs.insert((row, col));
                last_big = input_matrix[row][col] as i32;
            }
        }
    }

    for row in 0..nrows {
        let mut last_big = -1;
        for col in 0..ncols {
            if input_matrix[row][col] as i32 > last_big {
                pairs.insert((row, col));
                last_big = input_matrix[row][col] as i32;
            }
        }
    }

    for row in 0..nrows {
        let mut last_big = -1;
        for col in (0..=(ncols - 1)).rev() {
            if input_matrix[row][col] as i32 > last_big {
                pairs.insert((row, col));
                last_big = input_matrix[row][col] as i32;
            }
        }
    }
}

pub fn get_count(input: &str) -> u32 {
    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    let input_matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as u32 - '0' as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // println!("{:?}", input_matrix);
    get_sum(&input_matrix, &mut pairs);
    return pairs.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        r#"30373
25512
65332
33549
35390"#,
        21
    )]
    fn test_line_type_file(input: &str, out: u32) {
        assert_eq!(get_count(input), out);
    }

    #[test_case(INPUT, 1803)]
    fn test_input_file(input: &str, out: u32) {
        assert_eq!(get_count(input), out);
    }
}
