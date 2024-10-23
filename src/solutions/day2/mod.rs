mod input;
use crate::solver::Solver;
use crate::intcode::*;
use std::str::FromStr;

pub struct Problem;

fn get_program(program: &str) -> Vec<i64> {
    program
        .split(",")
        .filter_map(|c| i64::from_str(c).ok())
        .collect()
}

impl Solver for Problem {
    type Ans1 = i64;
    type Ans2 = i64;

    fn solution1(&self) -> i64 {
        let mut code = get_program(input::INPUT);
        code[1] = 12;
        code[2] = 2;
        let mut cpu: Cpu = Cpu::new(&code);
        cpu.run();
        cpu.code[0]
    }

    fn solution2(&self) -> i64 {
        let (mut n, mut v) = (0, 0);
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut code = get_program(input::INPUT);
                code[1] = noun;
                code[2] = verb;
                let mut cpu: Cpu = Cpu::new(&code);
                cpu.run();
                if cpu.code[0] == 19690720 {
                    (n, v) = (noun, verb);
                }
            }
        }
        100 * n + v
    }
}

