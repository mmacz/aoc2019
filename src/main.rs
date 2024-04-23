use std::env;
pub mod day1;
pub mod utils;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("File with input is not provided");
    }
    if let Ok(lines) = utils::read_lines(&args[1]) {
        println!("Sum of needed fuel is: {}", day1::solution(lines));
    }
    else {
        panic!("Cannot read the file");
    }
}

