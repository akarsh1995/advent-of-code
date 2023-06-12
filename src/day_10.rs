use std::collections::VecDeque;

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
        }
    }

    fn decode(&mut self) {
        self.current = self.instructions.pop_front().map(|i| (i, i.cycles()));
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
}
