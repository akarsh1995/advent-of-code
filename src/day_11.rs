use std::{fmt::Display, collections::VecDeque};

use itertools::Itertools;
use regex::Regex;

type WorryLevel = i32;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Operation,
    test: i32,
    throw_true: u32,
    throw_false: u32,
    times_inspected: u32, 
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.items.iter().join(", "), self.times_inspected)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Value {
    Old,
    Value(i32),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Value(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        if value == "old" {
            Self::Old
        } else {
            Self::Value(value.parse::<i32>().unwrap())
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Operation {
    Multiply { value: Value },
    Add { value: Value },
    Divide { value: Value },
    Subtract { value: Value },
}

struct KeepAwayGame {
    monkeys: Vec<Monkey>,
}

impl KeepAwayGame {
    fn times_inspected(&self) -> Vec<u32> {
        self.monkeys.iter().map(|m| m.times_inspected).collect_vec()
    }

    fn throw(&mut self) {
        for i in 0..self.monkeys.len() {
            println!("Monkey: {i}:");
            let mut worries = vec![];
            let op: Operation;
            let test: i32;
            let tt: u32;
            let tf: u32;
            {
                let monkey = &mut self.monkeys[i];
                op= monkey.operation.clone();
                test= monkey.test;
                tt= monkey.throw_true;
                tf= monkey.throw_false;
                let mut item = monkey.items.pop_front();
                while let Some(it) = item {
                    worries.push(it);
                    item = monkey.items.pop_front();
                }
            }
            for worry in worries {
                println!("\tMonkey inspects an item with a worry level of {worry}.");
                let to_throw_at = operate(worry, op.clone(), test, tt, tf);
                self.monkeys[to_throw_at.0 as usize].items.push_back(to_throw_at.1);
                self.monkeys[i].times_inspected += 1;
                println!("\t\tItem with worry level {} is thrown to monkey {}.",  to_throw_at.1, to_throw_at.0)
            }
        }
    }
}


impl Display for KeepAwayGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s =self.monkeys.iter().enumerate().map(|(i, m)| format!("Monkey {i}: {m}")).join("\n");
        write!(f, "{s}")
    }
}

fn operate(l: WorryLevel, operation: Operation, test: i32, if_true: u32, if_false: u32) -> (u32, i32) {
    let op_result = match operation {
        Operation::Multiply { value } => match value {
            Value::Old => {
                let res = l*l;
                println!("\t\tWorry level is multiplied by {l} to {res}");
                res
            },
            Value::Value(v) => {
                let res  = l*v;
                println!("\t\tWorry level is multiplied by {v} to {res}");
                res
            },
        },
        Operation::Add { value } => match value {
            Value::Old => {
                let res  = l+l;
                println!("\t\tWorry level increases by {l} to {res}");
                res
            },
            Value::Value(v) => {
                let res = v+l;
                println!("\t\tWorry level increases by {v} to {res}");
                res
            },
        },
        Operation::Divide { value } => match value {
            Value::Old => {
                let res  = l /l;
                println!("\t\tWorry level divided by {l} to {res}");
                res
            },
            Value::Value(v) => {
                let res = l/ v;
                println!("\t\tWorry level divided by {v} to {res}");
                res
            },
        },
        Operation::Subtract { value } => match value {
            Value::Old => {
                let res = l - l;
                println!("\t\tWorry level subtracted by {l} to {res}");
                res

            },
            Value::Value(v) => {
                let res = l - v;
                println!("\t\tWorry level subtracted by {v} to {res}");
                res
            },
        },
    };
    // let to_test = l * multiply_by;
    let divide_by = 3;
    let worry_level = op_result / divide_by;
    println!("\t\tMonkey gets bored with item. Worry level is divided by {divide_by} to {worry_level}.");
    (if worry_level % test == 0 {
        println!("\t\tCurrent worry level is divisible by {test}.");
        if_true
    } else {
        println!("\t\tCurrent worry level is not divisible by {test}.");
        if_false
    }, worry_level)
}

impl Monkey {
    fn parse(values: &str) -> Self {
        let mut items = VecDeque::new();
        let mut operation: Operation = Operation::Divide { value: 8.into() };
        let mut test: i32 = 0;
        let mut throw_true: u32 = 0;
        let mut throw_false: u32 = 0;

        let lines = values.lines();

        for (i, line) in lines.enumerate() {
            match i {
                0 => {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let mut captures = re.captures_iter(line);
                    captures.next().unwrap()[1].parse::<u32>().unwrap();
                }
                1 => {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let captures = re.captures_iter(line);
                    for m in captures {
                        items.push_back(m[1].parse::<WorryLevel>().unwrap());
                    }
                }
                2 => {
                    let re = Regex::new(r"(\*|\/|\+|\-)\s(\-?\d+|old)").unwrap();
                    let mut captures = re.captures_iter(line);
                    let match_ = captures.next().unwrap();

                    let by = match_[2].into();

                    let op = match &match_[1] {
                        "*" => Operation::Multiply { value: by },
                        "+" => Operation::Add { value: by },
                        "/" => Operation::Divide { value: by },
                        "-" => Operation::Subtract { value: by },
                        _ => panic!("Unknown operation"),
                    };
                    operation = op;
                }
                3 => {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let mut captures = re.captures_iter(line);
                    test = captures.next().unwrap()[1].parse::<i32>().unwrap();
                }
                4 => {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let mut captures = re.captures_iter(line);
                    throw_true = captures.next().unwrap()[1].parse::<u32>().unwrap();
                }
                5 => {
                    let re = Regex::new(r"(\d+)").unwrap();
                    let mut captures = re.captures_iter(line);
                    throw_false = captures.next().unwrap()[1].parse::<u32>().unwrap();
                }
                _ => panic!("Line exceeded"),
            }
        }
        Self {
            times_inspected: 0,
            items,
            operation,
            test,
            throw_true,
            throw_false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const MONKEY_SAMPLE_STRING: &'static str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#;

    #[test_case("  Starting items: 79, 98", vec![79, 98]; "test_parsing")]
    fn test_parsing(input: &str, output: Vec<WorryLevel>) {
        let re = Regex::new(r"(\d+)").unwrap();
        let mut o: Vec<WorryLevel> = vec![];
        for cap in re.captures_iter(input) {
            o.push((&cap[1]).parse::<WorryLevel>().unwrap());
        }
        assert_eq!(o, output);
    }

    #[test_case(MONKEY_SAMPLE_STRING, 
        Monkey {
            items: vec![79, 98].iter().map(|i| *i as i32).collect::<VecDeque<_>>(),
            operation: Operation::Multiply { value: 19.into() },
            test: 23,
            throw_true: 2,
            throw_false: 3 ,
            times_inspected: 0,
        } ; "test_parsing_monkey")]
    fn test_parsing_monkey(input: &str, output: Monkey) {
        assert_eq!(Monkey::parse(input), output);
    }

    #[test_case(include_str!("../data/year_2022__day_11"); "test_parsing_monkeys_sample")]
    fn test_parsing_monkeys_sample(input: &str) {
        for m in input.split("\n\n") {
            dbg!(Monkey::parse(m));
        }
    }

    #[test_case(include_str!("../data/year_2022__day_11"); "test_day_11")]
    fn test_day_11(input: &str) {
       let k = input.split("\n\n").map( |l| 
            Monkey::parse(l)
        ).collect::<Vec<_>>();
        let mut k = KeepAwayGame { monkeys: k };
        for _ in 0..20 {
            k.throw();
        }
        let mut ti = k.times_inspected();
        ti.sort();
        ti.reverse();
        println!("{:?}{}", ti, ti[..2].iter().fold(1, |x, y| x * y));
    }

    #[test_case(include_str!("../data/sample_day_11"); "test_sample_p1")]
    fn test_sample_p1(input: &str) {
       let k = input.split("\n\n").map( |l| 
            Monkey::parse(l)
        ).collect::<Vec<_>>();
        let mut k = KeepAwayGame { monkeys: k };
        for _ in 0..20 {
            k.throw();
        }
        let mut ti = k.times_inspected();
        ti.sort();
        ti.reverse();
        println!("{:?}{}", ti, ti[..2].iter().fold(1, |x, y| x * y));
    }

}
