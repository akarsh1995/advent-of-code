#[allow(unused)]
use std::{collections::HashSet, io::Read};

pub const INPUT: &'static str = include_str!("../data/year_2022__day_7");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stdout<'a> {
    File { name: &'a str, size: u32 },
    Dir { name: &'a str },
}

impl<'a> Stdout<'a> {
    fn new(output: &'a str) -> Self {
        let mut splitted = output.split_whitespace();
        let first = splitted.next().unwrap();
        let second = splitted.next().unwrap();
        match first.parse::<u32>() {
            Ok(n) => Self::File {
                name: second,
                size: n,
            },
            Err(e) => Self::Dir { name: second },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LineType<'line> {
    Command { cmd: &'line str, arg: &'line str },
    Stdout(&'line str),
}

impl<'line> LineType<'line> {
    fn new(line: &'line str) -> Self {
        if line.starts_with("$") {
            Self::Command {
                cmd: &line[2..4],
                arg: &line[5..],
            }
        } else {
            Self::Stdout(line)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    //
    #[test_case("$ cd /"; "line type cd /")]
    fn test_line_type_command(input: &str) {
        assert_eq!(
            LineType::new(input),
            LineType::Command {
                cmd: "cd",
                arg: "/"
            }
        );
    }

    #[test_case("dir e", "e"; "line type cd /")]
    fn test_line_type_dir(input: &str, dirname: &str) {
        assert_eq!(Stdout::new(input), Stdout::Dir { name: "e" },);
    }

    //
    #[test_case("29116 f", "f", 29116; "line type cd /")]
    fn test_line_type_file(input: &str, filename: &str, size: u32) {
        assert_eq!(
            Stdout::new(input),
            Stdout::File {
                name: "f",
                size: 29116
            }
        );
    }

    //     #[test_case(
    //         r#"$ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k
    // "#
    //     )]
    //     // #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz", 4)]
    //     // #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg", 4)]
    //     // #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)]
    //     // #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)]
    //     // #[test_case(2334, INPUT, 14)]
    //     fn test_find_marker(structure: &str) {
    //         // assert_eq!(res, get_marker_fixed_bitwise(s, w));
    // }
}
