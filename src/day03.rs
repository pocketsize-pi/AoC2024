use std::collections::VecDeque;
use aoc_2024::*;
use regex::Regex;


pub fn day03(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 03");


    let data = read_input(03, input_type, manual_name)?;

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut values = vec![];
    let mut final_score = 0;
    for row in &data {
        for cell in row {
            for (_, [f1, f2]) in re.captures_iter(&*cell).map(|c| c.extract()) {
                values.push((f1.parse::<i32>()?, f2.parse::<i32>()?));
                final_score += f1.parse::<i32>()? * f2.parse::<i32>()?;
            }
        }
    }


    // println!("{:?}", values);
    println!("Final value is: {}", final_score);
    // 843172 too low (because I hadn't used all the input data)
    // 9499483 also too low - still not using all the data! there are spaces, because of my input handling
    // 170778545 finally!

    Ok(())
}