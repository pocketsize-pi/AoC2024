use std::fs::File;
use std::io::{prelude::*, BufReader};

// INPUT PROCESSING
#[derive(PartialEq, Clone, Copy)]
pub enum InputType {
    Sample,
    Sample2,
    Data,
    Manual,
}

pub fn read_input(day: u8, input: InputType, manual_name: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {

    let file_name = match input {
        InputType::Sample => format!("data/day{:02}_sample.txt",day),
        InputType::Sample2 => format!("data/day{:02}_sample2.txt",day),
        InputType::Data=> format!("data/day{:02}_input.txt",day),
        InputType::Manual => format!("data/{}", manual_name),
    };
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let words: Vec<String> = line.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        data.push(words);
    }

    // println!("{:?}", data);
    Ok(data)
}

// TYPE CONVERSIONS
pub fn str_to_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn str_to_i64 (input: &str) -> i64 {
    input.parse::<i64>().unwrap()
}

pub fn str_to_i32 (input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

pub fn str_to_u16 (input: &str) -> u16 {
    input.parse::<u16>().unwrap()
}

pub fn str_to_u32 (input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

pub fn str_to_usize (input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

