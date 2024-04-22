use std::env;
pub mod day1;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("File with input is not provided");
    }

    // println!("Sum of needed fuel is: {}", day1::solution(&args[1]));
}


