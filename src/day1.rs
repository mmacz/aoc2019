use std::fs::File;
use std::io::{self};

fn fuel_counter(mass: i32) -> i32{
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    }
    fuel + fuel_counter(fuel)
}


pub fn solution(input: io::Lines<io::BufReader<File>>) -> i32
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
