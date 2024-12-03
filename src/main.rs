#![allow(clippy::zero_prefixed_literal)]

use std::env;
use aoc_2024::InputType;

pub mod day00;
pub mod day01;
pub mod day02;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, Advent of Code 2016!");

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        // No choice
        println!("Try again with a day")
    }
    else if args[0].parse::<u8>().is_ok() {
        let day = args[0].parse::<u8>().unwrap();
        let mut input_type = InputType::Sample;
        let mut manual_name = "not_used.txt";

        if args.len() > 1 {
            if args[1] == "s" {
                input_type = InputType::Sample;
            }
            else if args[1] == "s2" {
                // sample 2 hack, might need to reconfigure the main for next year
                input_type = InputType::Sample2;
            }
            else if args[1] == "d" {
                input_type = InputType::Data;
            }
            else if args[1].contains('.') {
                input_type = InputType::Manual;
                manual_name = args[1].as_str();
            }
        }

        match day {
            01 => day01::day01(input_type, manual_name)?,
            02 => day02::day02(input_type, manual_name)?,

            _others => day00::day00(input_type, manual_name)?
        }
    }
    else {
        println!("Not a number, try again")
    }

    Ok(())
}
