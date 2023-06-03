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

#[test]
fn test_day_2p1_sample() {
    let sample_string = "A Y\nB X\nC Z";
    println!("test_day_2p1_sample: {}", calc_total_score(sample_string));
    assert_eq!(15, calc_total_score(sample_string));
}

#[test]
fn test_day_2p1() {
    let some_string: String = std::fs::read_to_string("./data/day_2p1").unwrap();
    println!("{}", calc_total_score(&some_string));
}
