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

fn parse_code(code: &mut i32, digits: i32) -> i32 {
    let rem: i32 = *code % digits;
    *code = *code / digits;
    rem
}

fn get_operand_by_mode(code: &Vec<i32>, pc: usize, mode: i32, op_idx: usize) -> i32 {
    let mut op: i32;
    match mode {
        0 => {
            op = code[pc + op_idx];
            op = code[op as usize];
        }
        1 => {
            op = code[pc + op_idx];
        }
        _ => panic!("Unknown operand mode: {}", mode),
    }
    op
}

fn intcode_decode(code: &mut Vec<i32>) -> () {
    let mut pc: usize = 0;
    while pc < code.len() {
        let mut intcode: i32 = code[pc];
        let opcode: i32 = parse_code(&mut intcode, 100);
        let op1_mode: i32 = parse_code(&mut intcode, 10);
        let op2_mode: i32 = parse_code(&mut intcode, 10);
        let _op3_mode: i32 = parse_code(&mut intcode, 10);
        match opcode {
            1 => {
                // add
                let op1: i32 = get_operand_by_mode(&code, pc, op1_mode, 1);
                let op2: i32 = get_operand_by_mode(&code, pc, op2_mode, 2);
                let op3: i32 = get_operand_by_mode(&code, pc, 1, 3);
                code[op3 as usize] = op1 + op2;
                pc += 4;
            }
            2 => {
                // mul
                let op1: i32 = get_operand_by_mode(&code, pc, op1_mode, 1);
                let op2: i32 = get_operand_by_mode(&code, pc, op2_mode, 2);
                let op3: i32 = get_operand_by_mode(&code, pc, 1, 3);
                code[op3 as usize] = op1 * op2;
                pc += 4;
            }
            3 => {
                // ld
                let mut input: String = String::new();
                println!("Provide an input: ");
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Input not provided");
                let param: i32 = input.trim().parse().expect("Cannot parse the input");
                let op1: usize = get_operand_by_mode(&code, pc, 1, 1) as usize;
                code[op1] = param;
                pc += 2;
            }
            4 => {
                // rd
                let op1: usize = get_operand_by_mode(&code, pc, 1, 1) as usize;
                println!("Read: {}", code[op1]);
                pc += 2;
            }
            5 => {
                // jt
                let op1: i32 = get_operand_by_mode(&code, pc, op1_mode, 1);
                let op2: i32 = get_operand_by_mode(&code, pc, op2_mode, 2);
                if op1 != 0 {
                    pc = op2 as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                // jf
                let op1: i32 = get_operand_by_mode(&code, pc, op1_mode, 1);
                let op2: i32 = get_operand_by_mode(&code, pc, op2_mode, 2);
                if op1 == 0 {
                    pc = op2 as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                // lt
                let op1: i32 = get_operand_by_mode(&code, pc, op1_mode, 1);
                let op2: i32 = get_operand_by_mode(&code, pc, op2_mode, 2);
                let op3: i32 = get_operand_by_mode(&code, pc, 1, 3);
                code[op3 as usize] = if op1 < op2 { 1 } else { 0 };
                pc += 4;
            }
            8 => {
                // eq
                let op1: i32 = get_operand_by_mode(&code, pc, op1_mode, 1);
                let op2: i32 = get_operand_by_mode(&code, pc, op2_mode, 2);
                let op3: i32 = get_operand_by_mode(&code, pc, 1, 3);
                code[op3 as usize] = if op1 == op2 { 1 } else { 0 };
                pc += 4;
            }

            99 => break,
            _ => panic!("Unknown opcode {}", code[pc]),
        }
    }
}

fn solution(input: io::Lines<io::BufReader<File>>) -> () {
    for line in input.flatten() {
        // always 1 line
        let mut intcode: Vec<i32> = line
            .split(",")
            .filter_map(|c| i32::from_str(c).ok())
            .collect();
        intcode_decode(&mut intcode);
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
