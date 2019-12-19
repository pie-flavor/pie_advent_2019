use aoc_runner_derive::*;

#[aoc_generator(day2)]
fn generate_intcode(input: &str) -> Vec<i32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn run_intcode(intcode: &[i32]) -> i32 {
    let mut program = intcode.to_owned();
    program[1] = 12;
    program[2] = 2;
    let mut computer = IntcodeComputer {
        memory: program,
        pc: 0,
    };
    computer.run();
    computer.memory[0]
}

#[aoc(day2, part2)]
fn check_input(intcode: &[i32]) -> i32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = intcode.to_owned();
            program[1] = noun;
            program[2] = verb;
            let mut computer = IntcodeComputer {
                memory: program,
                pc: 0,
            };
            computer.run();
            if computer.memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No applicable input")
}

struct IntcodeComputer {
    memory: Vec<i32>,
    pc: usize,
}

impl IntcodeComputer {
    fn move_next(&mut self) -> bool {
        let opcode = self.memory[self.pc];
        match opcode {
            1 => {
                let (idx1, idx2, idx_res) = (
                    self.memory[self.pc + 1] as usize,
                    self.memory[self.pc + 2] as usize,
                    self.memory[self.pc + 3] as usize,
                );
                self.memory[idx_res] = self.memory[idx1] + self.memory[idx2];
                self.pc += 4;
                true
            }
            2 => {
                let (idx1, idx2, idx_res) = (
                    self.memory[self.pc + 1] as usize,
                    self.memory[self.pc + 2] as usize,
                    self.memory[self.pc + 3] as usize,
                );
                self.memory[idx_res] = self.memory[idx1] * self.memory[idx2];
                self.pc += 4;
                true
            }
            99 => false,
            _ => panic!("Invalid opcode {}", opcode),
        }
    }
    fn run(&mut self) {
        while self.move_next() {}
    }
}
