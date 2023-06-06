struct Input<'a>(&'a str);

impl<'a> Input<'a> {
    fn get_move(&self) -> Move {
        match self.0 {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!(""),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum OutCome {
    Win,
    Loss,
    Draw,
}

impl Move {
    fn get_move_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_outcome(&self, opponents_move: &Self) -> OutCome {
        match (self, opponents_move) {
            (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper)
            | (Move::Paper, Move::Rock) => OutCome::Win,
            (a, b) => {
                if a == b {
                    OutCome::Draw
                } else {
                    OutCome::Loss
                }
            }
        }
    }

    fn get_win_score(&self, opponents_move: &Self) -> u32 {
        self.get_move_score()
            + match self.get_outcome(opponents_move) {
                OutCome::Win => 6,
                OutCome::Loss => 0,
                OutCome::Draw => 3,
            }
    }
}

fn calc_total_score<'a>(in_str: &'a str) -> u32 {
    let mut sc = 0;
    for s in in_str.split("\n") {
        if s == "" {
            continue;
        }
        let mut x = s.split(" ");
        let o = Input(x.next().unwrap());
        let p = Input(x.next().unwrap());
        let mv1 = p.get_move();
        let mv2 = o.get_move();
        sc += mv1.get_win_score(&mv2);
    }
    sc
}

#[cfg(test)]
mod tests_p1 {
    use super::*;
    #[test]
    fn test_sample() {
        let sample_string = "A Y\nB X\nC Z";
        println!("test_day_2p1_sample: {}", calc_total_score(sample_string));
        assert_eq!(15, calc_total_score(sample_string));
    }

    #[test]
    fn test_input() {
        let some_string: String = std::fs::read_to_string("./data/day_2p1").unwrap();
        println!("{}", calc_total_score(&some_string));
    }
}

struct Action<'a>(&'a str);

struct Input2<'a>(&'a str);

fn get_win_move(opponents_move: &Move) -> Move {
    match opponents_move {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn get_losing_move(opponents_move: &Move) -> Move {
    match opponents_move {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

impl<'a> Input2<'a> {
    fn target(&self) -> OutCome {
        let s = self.0.split(" ");
        match s.last().unwrap() {
            "Y" => OutCome::Draw,
            "X" => OutCome::Loss,
            "Z" => OutCome::Win,
            _ => panic!("Wrong outcome"),
        }
    }

    fn opponent_move(&self) -> Move {
        let mut s = self.0.split(" ");
        match s.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Wrong move"),
        }
    }

    fn get_target_move(&self) -> Move {
        match self.target() {
            OutCome::Win => get_win_move(&self.opponent_move()),
            OutCome::Loss => get_losing_move(&self.opponent_move()),
            OutCome::Draw => self.opponent_move(),
        }
    }

    fn get_score(&self) -> u32 {
        let my_move = &self.get_target_move();
        my_move.get_win_score(&self.opponent_move())
    }
}

#[cfg(test)]
mod tests_p2 {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_string = "A Y\nB X\nC Z";
        let mut s = 0;
        for k in sample_string.split("\n") {
            s += Input2(k).get_score();
        }
        assert_eq!(12, s);
    }

    #[test]
    fn test_input() {
        let some_string: String = std::fs::read_to_string("./data/day_2p1").unwrap();
        let mut s = 0;
        for k in some_string.split("\n") {
            if k != "" {
                s += Input2(k).get_score();
            }
        }
        println!("test_input: {}", s);
    }
}
