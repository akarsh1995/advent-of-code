const INPUT: &'static str = include_str!("../../data/year_2022__day_4");

#[derive(Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, s: Range) -> bool {
        (self.start..=self.end).contains(&s.start) && (self.start..=self.end).contains(&s.end)
    }

    fn contains_partial(&self, s: Range) -> bool {
        (self.start..=self.end).contains(&s.start) || (self.start..=self.end).contains(&s.end)
    }

    fn new(s: &str) -> Self {
        let j = s.split_once("-").unwrap();
        Range {
            start: j.0.parse().unwrap(),
            end: j.1.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_1_sample() {
        let dk = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        let mut pairs = 0;
        for entry in dk.lines() {
            let (r1, r2) = entry.split_once(',').unwrap();
            let (r1, r2) = (Range::new(r1), Range::new(r2));
            if r1.contains(r2) || r2.contains(r1) {
                pairs += 1;
            }
        }
        assert_eq!(pairs, 2);
    }

    #[test]
    fn test_p_1() {
        let mut pairs = 0;
        for line in INPUT.lines() {
            let (r1, r2) = line.split_once(',').unwrap();
            let (r1, r2) = (Range::new(r1), Range::new(r2));
            if r1.contains(r2) || r2.contains(r1) {
                pairs += 1;
            }
        }
        dbg!(pairs);
    }

    #[test]
    fn test_p_2_sample() {
        let dk = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        let mut pairs = 0;
        for line in dk.lines() {
            let (r1, r2) = line.split_once(',').unwrap();
            let (r1, r2) = (Range::new(r1), Range::new(r2));
            if r1.contains_partial(r2) || r2.contains_partial(r1) {
                pairs += 1;
            }
        }
        assert_eq!(pairs, 4);
    }

    #[test]
    fn test_p_2() {
        let mut pairs = 0;
        for line in INPUT.lines() {
            let (r1, r2) = line.split_once(',').unwrap();
            let (r1, r2) = (Range::new(r1), Range::new(r2));
            if r1.contains_partial(r2) || r2.contains_partial(r1) {
                pairs += 1;
            }
        }
        dbg!(pairs);
    }
}
