use std::collections::VecDeque;
use std::env;
use std::fs;
use std::string::String;
use std::str::FromStr;
use cpu::Cpu;
use itertools::Itertools;

pub mod sanity_inputs;
pub mod cpu;

fn get_intcode_from_str(str_code: &str) -> Vec<i32> {
    str_code.split(",").filter_map(|c| i32::from_str(c).ok()).collect()
}

fn solution1(file: &String) -> i32 {
    let mut max_out = 0;
    let permutations = vec!{0, 1, 2, 3, 4}.into_iter().permutations(5);
    let code: Vec<i32> = get_intcode_from_str(&file);

    for phases in permutations {
        let mut out: i32 = 0;
        for phase in phases {
            let mut amp: cpu::Cpu = cpu::Cpu::new(phase, &code);
            let mut done: bool = false;
            while !done {
                let input: i32 = out;
                amp.process(input);
                done = amp.done;
            }
            out = amp.out;
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
    let code: Vec<i32> = get_intcode_from_str(&file);
    

    for phases in permutations {
        let mut amp_a: Cpu = Cpu::new(phases[0], &code);
        let mut amp_b: Cpu = Cpu::new(phases[1], &code);
        let mut amp_c: Cpu = Cpu::new(phases[2], &code);
        let mut amp_d: Cpu = Cpu::new(phases[3], &code);
        let mut amp_e: Cpu = Cpu::new(phases[4], &code);

        // Forced A input
        amp_e.out = 0;
        while  !amp_a.done 
            && !amp_b.done
            && !amp_c.done
            && !amp_d.done
            && !amp_e.done {
                amp_a.process(amp_e.out);
                amp_b.process(amp_a.out);
                amp_c.process(amp_b.out);
                amp_d.process(amp_c.out);
                amp_e.process(amp_d.out);
        }

        let out: i32 = amp_e.out;
        if out > max_out {
            max_out = out;
        }

    }
    max_out
}

fn main() {
    assert_eq!(43210, solution1(&sanity_inputs::sanity1()));
    assert_eq!(139629729, solution2(&sanity_inputs::sanity4()));

    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //     panic!("File with input is not provided");
    // }
    // let input: String = fs::read_to_string(&args[1]).unwrap();
    //println!("Answer 1: {}", solution1(&input));
}

