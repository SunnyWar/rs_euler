mod problems;
use problems::to_25::*;
use std::env;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<u32>() {
            Ok(n) => match n {
                1 => problem_1(),
                2 => problem_2(),
                3 => problem_3(),
                4 => problem_4(),
                5 => problem_5(),
                6 => problem_6(),
                7 => problem_7(),
                8 => problem_8(),
                9 => problem_9(),
                10 => problem_10(),
                11 => problem_11(),
                12 => problem_12(),
                13 => problem_13(),
                14 => problem_14(),
                15 => problem_15(),
                16 => problem_16(),
                17 => problem_17(),
                18 => problem_18(),
                _ => println!("No problem associated with this number."),
            },
            Err(_) => println!("Please provide a valid number."),
        }
    } else {
        println!("Please provide a problem number as a command line argument.");
    }
}
