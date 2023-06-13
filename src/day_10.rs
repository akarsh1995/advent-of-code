use std::{collections::VecDeque, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    NoOp,
    AddX { value: i32 },
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX { value: _ } => 2,
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value.starts_with("noop") {
            true => Self::NoOp,
            false => Self::AddX {
                value: value
                    .chars()
                    .skip(5)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CPU {
    current: Option<(Instruction, u32)>,
    register_x: i32,
    instructions: VecDeque<Instruction>,
    cycles: u32,
    display: Vec<Vec<char>>,
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = self
            .display
            .iter()
            .map(|c| c.iter().collect::<String>())
            .join("\n");
        write!(f, "{k}")
    }
}

impl CPU {
    fn new(input: &str) -> Self {
        let inss = input
            .lines()
            .map(|l| Instruction::from(l))
            .collect::<VecDeque<_>>();
        Self {
            current: None,
            cycles: 0,
            register_x: 1,
            instructions: inss,
            display: (0..6)
                .map(|_| (0..40).map(|_| '.').collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        }
    }

    fn decode(&mut self) {
        self.current = self.instructions.pop_front().map(|i| (i, i.cycles()));
    }

    fn draw(&mut self) {
        let nth_row = ((self.cycles / 40) % 6) as usize;
        let display_row = &mut self.display[nth_row];
        if self.register_x == -1 {
            display_row[(self.cycles % 40) as usize] = '#';
        }
        if self.register_x >= 0 && self.register_x <= 39 {
            let c_mask = cycle_mask(self.cycles % 40);
            let s_mask = sprite_value(self.register_x as u32);
            if (c_mask & s_mask) > 0 {
                display_row[(self.cycles % 40) as usize] = '#';
            } else {
                display_row[(self.cycles % 40) as usize] = '.';
            }
        } else {
            display_row[(self.cycles % 40) as usize] = '.';
        }
    }

    fn step(&mut self) -> bool {
        if let Some((i, cycles)) = &mut self.current {
            if *cycles == 0 {
                match i {
                    Instruction::NoOp => {}
                    Instruction::AddX { value } => {
                        self.register_x += *value;
                    }
                }
                self.decode();
            } else {
                *cycles -= 1;
                self.cycles += 1;
            }
            true
        } else {
            false
        }
    }

    fn get_running_cycle(&self) -> u32 {
        self.cycles
    }
}

const DISPLAY_MASK: u64 = 0b1111111111111111111111111111111111111111;

fn cycle_mask(cycle: u32) -> u64 {
    (0b1000000000000000000000000000000000000000 >> (cycle % 40)) & DISPLAY_MASK
}

fn sprite_value(pos: u32) -> u64 {
    (0b11100000000000000000000000000000000000000 >> pos) & DISPLAY_MASK
}

#[cfg(test)]
mod bit_tests {

    pub const INPUT: &'static str = include_str!("../data/year_2022__day_10");
    use super::*;

    #[test]
    fn test_display() {
        let cpu = CPU::new(INPUT);
        println!("{}", cpu);
    }

    #[test]
    fn test() {
        assert_eq!(
            format!("{:040b}", sprite_value(0)),
            "1100000000000000000000000000000000000000"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(1)),
            "1110000000000000000000000000000000000000"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(38)),
            "0000000000000000000000000000000000000111"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(39)),
            "0000000000000000000000000000000000000011"
        );
        assert_eq!(
            format!("{:040b}", sprite_value(40)),
            "0000000000000000000000000000000000000001"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("addx -5", Instruction::AddX{ value: -5 })]
    fn test_instruction_parse(input: &str, output: Instruction) {
        assert_eq!(<&str as Into<Instruction>>::into(input), output);
    }

    #[test_case("noop\naddx 3\naddx -5", 5; "test_cpu_instructions_cycle_count")]
    fn test_cpu_instructions_cycle_count(input: &str, cycle_count: u32) {
        let mut cpu = CPU::new(input);
        cpu.decode();
        loop {
            if !cpu.step() {
                break;
            };
        }
        assert_eq!(cpu.get_running_cycle(), cycle_count);
        assert_eq!(cpu.register_x, -1);
    }

    #[test_case(include_str!("../data/sample_day_10"), 13140; "test_cpu_instructions_signal_strength_sample")]
    fn test_cpu_instructions_signal_strength_sample(input: &str, signal_strength: i32) {
        let mut ss: i32 = 0;
        let mut next_cycle = 20;
        let mut cpu = CPU::new(input);
        cpu.decode();
        loop {
            if !cpu.step() {
                break;
            } else {
                if (cpu.get_running_cycle()) == next_cycle {
                    ss += cpu.register_x * cpu.get_running_cycle() as i32;
                    next_cycle += 40;
                }
            };
        }
        assert_eq!(ss, signal_strength);
    }

    #[test_case(include_str!("../data/year_2022__day_10"); "test_cpu_instructions_signal_strength_p_1")]
    fn test_cpu_instructions_signal_strength_p_1(input: &str) {
        let mut ss: i32 = 0;
        let mut next_cycle = 20;
        let mut cpu = CPU::new(input);
        cpu.decode();
        loop {
            if !cpu.step() {
                break;
            } else {
                if (cpu.get_running_cycle()) == next_cycle {
                    ss += cpu.register_x * cpu.get_running_cycle() as i32;
                    next_cycle += 40;
                }
            };
        }
        dbg!(ss);
    }

    #[test_case(include_str!("../data/sample_day_10"); "test_cpu_instructions_display_p_2_sample")]
    fn test_cpu_instructions_display_p_2_sample(input: &str) {
        // let mut ss: i32 = 0;
        // let mut next_cycle = 20;
        let mut cpu = CPU::new(input);
        cpu.draw();
        cpu.decode();
        loop {
            if !cpu.step() {
                break;
            } else {
                cpu.draw();
            }
        }
        println!("{}", &cpu);
    }

    #[test_case(include_str!("../data/year_2022__day_10"); "test_cpu_instructions_display_p_2")]
    fn test_cpu_instructions_display_p_2(input: &str) {
        // let mut ss: i32 = 0;
        // let mut next_cycle = 20;
        let mut cpu = CPU::new(input);
        cpu.draw();
        cpu.decode();
        loop {
            if !cpu.step() {
                break;
            } else {
                cpu.draw();
            }
        }
        println!("{}", &cpu);
        // dbg!(ss);
    }
}
