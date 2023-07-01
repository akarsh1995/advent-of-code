#[allow(unused)]
use std::{collections::HashSet, io::Read};

pub const INPUT: &'static str = include_str!("../../data/year_2022__day_6");

pub fn get_marker(sample_string: &str, window: usize) -> u32 {
    for i in 0..sample_string.len() - window {
        if sample_string[i..i + window]
            .chars()
            .collect::<HashSet<_>>()
            .len()
            == window
        {
            return (i + window) as u32;
        }
    }

    1 as u32
}

pub fn get_all_bits_sum(n: usize) -> usize {
    let mut n = n;
    let mut s = 0;
    while n > 0 {
        s += n & 1;
        n >>= 1;
    }
    return s;
}

trait LowercaseLetter {
    fn to_u32_for_bitset(&self) -> u32;
}

impl LowercaseLetter for u8 {
    fn to_u32_for_bitset(&self) -> u32 {
        assert!(self.is_ascii_lowercase());
        1 << (*self as u32 - 'a' as u32)
    }
}

pub fn get_marker_fixed_bitwise(s: &str, w: usize) -> usize {
    s.as_bytes()
        .windows(w)
        .position(|window| {
            window
                .iter()
                .map(|c| c.to_u32_for_bitset())
                .fold(0, |acc, x| acc | x)
                .count_ones() as usize
                == w
        })
        .map(|pos| pos + w)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz", 4)]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg", 4)]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)]
    #[test_case(2334, INPUT, 14)]
    fn test_find_marker(res: usize, s: &str, w: usize) {
        assert_eq!(res, get_marker_fixed_bitwise(s, w));
    }
}
