use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn fuel_counter(mass: i32) -> i32{
    let fuel = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    }
    fuel + fuel_counter(fuel)
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn solution<P>(filename: P) -> i32
where P: AsRef<Path>, {
    assert!(fuel_counter(14) == 2, "14: fuel calculated {}", fuel_counter(14));
    assert!(fuel_counter(1969) == 966, "1969: fuel calculated {}", fuel_counter(1969));
    assert!(fuel_counter(100756) == 50346, "100756: fuel calculated {}", fuel_counter(100756));

    let mut total_fuel: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mass = line.parse::<i32>().unwrap();
            
            total_fuel += fuel_counter(mass);
        }
    }
    else {
        panic!("Cannot read file")
    }
    total_fuel
}
