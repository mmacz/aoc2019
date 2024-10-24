use cpu::CpuStatus;
use itertools::Itertools;
use std::env;
use std::fs;
use std::str::FromStr;
use std::string::String;

pub mod cpu;
pub mod sanity_inputs;

fn get_intcode_from_str(str_code: &str) -> Vec<i32> {
    str_code
        .split(",")
        .filter_map(|c| i32::from_str(c).ok())
        .collect()
}

fn solution1(str_code: &String) -> i32 {
    let code = get_intcode_from_str(str_code);
    [0, 1, 2, 3, 4]
        .into_iter()
        .permutations(5)
        .map(|phases| {
            phases.iter().fold(0, |prev, &i| {
                let mut amp = cpu::Cpu::new(&code);
                amp.push_input(i);
                amp.push_input(prev);
                amp.process_for_output()
            })
        })
        .max()
        .unwrap()
}

fn solution2(str_code: &String) -> i32 {
    let code = get_intcode_from_str(str_code);
    [5, 6, 7, 8, 9]
        .into_iter()
        .permutations(5)
        .map(|phases| {
            let mut amps = vec![cpu::Cpu::new(&code); 5];
            for i in 0..5 {
                amps[i].push_input(phases[i]);
            }
            let mut input: i32 = 0;
            for i in (0..5).cycle() {
                amps[i].push_input(input);
                match amps[i].process() {
                    CpuStatus::Output(o) => input = o,
                    CpuStatus::Finished => return input,
                    CpuStatus::WaitForInput => break,
                    CpuStatus::Running => continue,
                }
            }
            unreachable!()
        })
        .max()
        .unwrap()
}

fn main() {
    assert_eq!(43210, solution1(&sanity_inputs::sanity1()));
    assert_eq!(139629729, solution2(&sanity_inputs::sanity4()));

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("File with input is not provided");
    }
    let input: String = fs::read_to_string(&args[1]).unwrap();
    println!("Answer 1: {}", solution1(&input));
    println!("Answer 2: {}", solution2(&input));
}
