#![allow(clippy::zero_prefixed_literal, unused_parens)]

use std::env;
use aoc_2024::InputType;

pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, Advent of Code 2024!");

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
            03 => day03::day03(input_type, manual_name)?,
            04 => day04::day04(input_type, manual_name)?,
            05 => day05::day05(input_type, manual_name)?,
            06 => day06::day06(input_type, manual_name)?,
            07 => day07::day07(input_type, manual_name)?,
            08 => day08::day08(input_type, manual_name)?,
            09 => day09::day09(input_type, manual_name)?,
            10 => day10::day10(input_type, manual_name)?,
            11 => day11::day11(input_type, manual_name)?,
            12 => day12::day12(input_type, manual_name)?,
            13 => day13::day13(input_type, manual_name)?,
            14 => day14::day14(input_type, manual_name)?,
            15 => day15::day15(input_type, manual_name)?,
            16 => day16::day16(input_type, manual_name)?,

            _others => day00::day00(input_type, manual_name)?
        }
    }
    else {
        println!("Not a number, try again")
    }

    Ok(())
}
