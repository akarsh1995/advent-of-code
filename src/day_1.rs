use std::fs::read_to_string;

fn sol_1p1(x: &str) -> u32 {
    let splitted = x.split("\n");
    let mut max = 0;
    let mut current_elf = 0;
    for line in splitted {
        if line == "" {
            if current_elf > max {
                max = current_elf;
            }
            current_elf = 0
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    if current_elf > max {
        max = current_elf;
    }
    max
}

fn sol_1p2(x: &str) -> u32 {
    let splitted = x.split("\n");
    let mut current_elf = 0;
    let mut v = vec![];
    for line in splitted {
        if line == "" {
            v.push(current_elf);
            current_elf = 0
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    v.push(current_elf);
    v.sort();
    return v[v.len() - 3..v.len()].iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1p1() {
        let x: String =
            read_to_string("/home/akarshj/Programming/advent_of_code/data/input_1").unwrap();
        println!("Answer: {}", sol_1p1(&x));
    }

    #[test]
    fn test_1p1() {
        let x = "5\n6\n7\n\n5";
        assert_eq!(sol_1p1(x), 18);
    }

    #[test]
    fn test_1p2() {
        let x: String =
            read_to_string("/home/akarshj/Programming/advent_of_code/data/input_1").unwrap();
        println!("Answer: {:?}", sol_1p2(&x));
    }
}
