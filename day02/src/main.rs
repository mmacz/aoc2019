use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn intcode_decode(code: &mut Vec<i32>) -> Vec<i32> {
    let mut idx = 0;
    while idx < code.len() {
        match code[idx] {
            1 => {
                let dst = code[idx + 3] as usize;
                code[dst] = code[code[idx + 1] as usize] + code[code[idx + 2] as usize];
            }
            2 => {
                let dst = code[idx + 3] as usize;
                code[dst] = code[code[idx + 1] as usize] * code[code[idx + 2] as usize];
            }
            99 => break,
            _ => panic!("Unknown opcode"),
        }
        idx += 4;
    }
    return code.clone();
}

fn solution(input: io::Lines<io::BufReader<File>>) -> () {
    for line in input.flatten() {
        // always 1 line
        let intcode: Vec<i32> = line
            .split(",")
            .filter_map(|c| i32::from_str(c).ok())
            .collect();
        // here 1202 alarm signal
        // intcode[1] = 12;
        // intcode[2] = 2;
        for noun in 0..99 {
            for verb in 0..99 {
                let mut program = intcode.clone();
                program[1] = noun;
                program[2] = verb;
                intcode_decode(&mut program);

                if program[0] == 19690720 {
                    // print params that matches requirements
                    println!("noun: {}, verb: {}, prog[0]: {}", noun, verb, program[0]);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("File with input is not provided");
    }
    if let Ok(lines) = read_lines(&args[1]) {
        solution(lines);
    }
}
