use aoc_2024::*;

pub fn day01(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 01");


    let data = read_input(01, input_type, manual_name)?;
    let mut data0 = Vec::new();
    let mut data1 = Vec::new();
    for row in &data {
        // println!("{:?}", row);
        data0.push(str_to_i32(&row[0]));
        data1.push(str_to_i32(&row[1]));
    }

    data0.sort();
    data1.sort();

    let mut total_distance = 0;
    for i in 0..data0.len() {
        total_distance += (data0[i] - data1[i]).abs();
    }

    println!("The total distance is {}", total_distance);
    // 3714264

    Ok(())
}