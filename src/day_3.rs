use std::collections::HashSet;
fn get_ord_val(s: char) -> u8 {
    if s as u8 >= 97 {
        s as u8 - 96
    } else {
        s as u8 - 38
    }
}

fn get_ordval_from_string(s: &str) -> u8 {
    let ruck_size = s.len() / 2;
    let mut track_map = HashSet::new();

    for (i, k) in s.chars().enumerate() {
        if track_map.contains(&k) && i > (ruck_size - 1) {
            if k as u8 >= 97 {
                return k as u8 - 96;
            } else {
                return k as u8 - 38;
            }
        } else if i <= (ruck_size - 1) {
            track_map.insert(k);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day_3::get_ordval_from_string;

    use super::three_lines_common;

    #[test]
    fn test_ordval() {
        let f = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let mut s = 0;
        for k in f.split("\n") {
            s += get_ordval_from_string(k) as u32;
        }
        assert_eq!(s, 157);
    }

    #[test]
    fn test_p_1() {
        let f: &str = include_str!("../data/year_2022__day_3");
        let mut s = 0;
        for k in f.split("\n") {
            s += get_ordval_from_string(k) as u32;
        }
        println!("{}", s);
    }

    #[test]
    fn test_p_2_sample() {
        let f: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"#;
        let mut s = f.split("\n");
        let res = three_lines_common(s.next().unwrap(), s.next().unwrap(), s.next().unwrap());
        assert_eq!(res, 18);
    }

    #[test]
    fn test_p_2() {
        let f: &str = include_str!("../data/year_2022__day_3");
        let mut total = 0;
        let mut s = f.lines();
        let mut fl = s.next();
        let mut sl = s.next();
        let mut tl = s.next();

        loop {
            total += three_lines_common(fl.unwrap(), sl.unwrap(), tl.unwrap()) as u32;
            fl = s.next();
            sl = s.next();
            tl = s.next();
            if fl.is_none() {
                break;
            }
        }
        dbg!(total);
    }
}

fn three_lines_common(s1: &str, s2: &str, s3: &str) -> u8 {
    let s1_ = s1.chars().collect::<HashSet<_>>();
    let s2_ = s2.chars().collect::<HashSet<_>>();
    let s3_ = s3.chars().collect::<HashSet<_>>();
    let k = s1_
        .intersection(&s2_)
        .map(|x| x.clone())
        .collect::<HashSet<_>>();
    let common = k.intersection(&s3_).collect::<HashSet<_>>();
    return get_ord_val(**common.iter().collect::<Vec<_>>()[0]);
}
