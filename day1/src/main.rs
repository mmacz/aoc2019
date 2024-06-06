use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fuel_counter(mass: i32) -> i32{
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    }
    fuel + fuel_counter(fuel)
}

fn solution(input: io::Lines<io::BufReader<File>>) -> i32
{
    assert!(fuel_counter(14) == 2, "14: fuel calculated {}", fuel_counter(14));
    assert!(fuel_counter(1969) == 966, "1969: fuel calculated {}", fuel_counter(1969));
    assert!(fuel_counter(100756) == 50346, "100756: fuel calculated {}", fuel_counter(100756));

    let mut total_fuel: i32 = 0;
    for line in input.flatten() {
        let mass = line.parse::<i32>().unwrap();
        total_fuel += fuel_counter(mass);
    }
    total_fuel
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("File with input is not provided");
    }
    if let Ok(lines) = read_lines(&args[1]) {
        println!("Answer: {}", solution(lines))
    }
}