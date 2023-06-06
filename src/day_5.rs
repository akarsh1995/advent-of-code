const INPUT: &'static str = include_str!("../data/year_2022__day_5");

struct Move {
    n: u32,
    from: u32,
    to: u32,
}

impl Move {
    fn from_str(m: &str) -> Self {
        let m = m.split_whitespace().collect::<Vec<_>>();
        Move {
            n: m[1].parse().unwrap(),
            from: m[3].parse().unwrap(),
            to: m[5].parse().unwrap(),
        }
    }
}

fn read_stacks() -> Vec<Vec<char>> {
    let mut v = vec![];
    for col in 0..9 {
        let mut lines = INPUT.lines();
        let n_char = 1 + col * 4;
        let mut in_v = vec![];
        for row in 0..8 {
            let char = lines.next().unwrap().chars().nth(n_char).unwrap();

            if char != ' ' {
                in_v.push(char);
            }
        }
        in_v.reverse();
        v.push(in_v);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_1() {
        let mut rs = read_stacks();
        for line in INPUT.lines() {
            if !line.starts_with("m") {
                continue;
            } else {
                let mv = Move::from_str(line);
                for _ in 0..mv.n {
                    let popped = rs[mv.from as usize - 1].pop().unwrap();
                    rs[mv.to as usize - 1].push(popped)
                }
            }
        }
        dbg!(rs.iter().map(|k| k.last().unwrap()).collect::<String>());
    }

    #[test]
    fn test_p_2() {
        let mut rs = read_stacks();
        for line in INPUT.lines() {
            if !line.starts_with("m") {
                continue;
            } else {
                let mv = Move::from_str(line);
                let mut temp_move = vec![];
                for _ in 0..mv.n {
                    let popped = rs[mv.from as usize - 1].pop().unwrap();
                    temp_move.push(popped);
                }
                temp_move.reverse();
                rs[mv.to as usize - 1].extend(temp_move.iter());
            }
        }
        dbg!(rs.iter().map(|k| k.last().unwrap()).collect::<String>());
    }
}
