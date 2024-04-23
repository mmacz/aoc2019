use std::fs::File;
use std::io::{self};
use std::str::FromStr;

fn intcode_decode(code: &mut Vec<i32>) -> Vec<i32> {
    let mut idx = 0;
    while idx < code.len() {
        match code[idx] {
            1 => {
                let dst = code[idx + 3] as usize;
                code[dst] = code[code[idx + 1] as usize] + code[code[idx + 2] as usize];
            },
            2 => {
                let dst = code[idx + 3] as usize;
                code[dst] = code[code[idx + 1] as usize] * code[code[idx + 2] as usize];
            },
            99 => break,
            _ => panic!("Unknown opcode"),
        }
        idx += 4;
    }
    return code.clone();
}

pub fn solution(input: io::Lines<io::BufReader<File>>) -> ()
{
    for line in input.flatten() {
        // always 1 line
        let intcode: Vec<i32> = line.split(",").filter_map(|c| i32::from_str(c).ok()).collect();
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
