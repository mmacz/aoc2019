use std::str::FromStr;

pub mod cpu;
pub mod sanity_inputs;
pub mod input;

fn get_intcode_from_str(str_code: &str) -> Vec<i32> {
    str_code.split(",").filter_map(|c| i32::from_str(c).ok()).collect()
}

fn solution1(input: &str) -> () {
    let code = get_intcode_from_str(input);
    let mut cpu = cpu::Cpu::new(&code);
    cpu.run();
}

fn solution2(input: &str) -> () {
}

fn main() {
    solution1(&sanity_inputs::sanity1());
    solution1(&sanity_inputs::sanity2());
    solution1(&sanity_inputs::sanity3());
}

