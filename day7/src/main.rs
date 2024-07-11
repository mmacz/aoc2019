use std::env;
use std::fs;
use std::string::String;
use std::str::FromStr;
use std::collections::VecDeque;
use itertools::Itertools;

pub mod computer;
pub mod sanity_inputs;

fn get_intcode_from_str(str_code: &str) -> Vec<i32> {
    str_code.split(",").filter_map(|c| i32::from_str(c).ok()).collect()
}

fn solution1(file: &String) -> i32 {
    let mut max_out = 0;
    let permutations = vec!{0, 1, 2, 3, 4}.into_iter().permutations(5);

    for phases in permutations {
        let mut out: i32 = 0;
        for phase in phases {
            let mut inputs: VecDeque<i32> = vec!{phase, out}.into();
            let mut code: Vec<i32> = get_intcode_from_str(&file);
            out = computer::intcode_decode(&mut code, &mut inputs);
        }
        if out > max_out {
            max_out = out;
        }
    }
    max_out
}

fn solution2(file: &String) -> i32 {
    let mut max_out: i32 = 0;
    let permutations = vec!{5, 6, 7, 8, 9}.into_iter().permutations(5);

    for phases in permutations {
        let out: i32 = 0;
        for phase in phases {
        }
        if out > max_out {
            max_out = out;
        }
    }
    max_out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("File with input is not provided");
    }
    let file: String = fs::read_to_string(&args[1]).unwrap();

    println!("Answer 1: {}", solution1(&file));
    println!("Answer 2: {}", solution2(&sanity_inputs::sanity4()));
}

