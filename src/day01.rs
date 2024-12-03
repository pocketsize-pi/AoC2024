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

    // Part 2
    // The fact that we sorted them earlier doesn't matter
    // I feel there is a clever way to do this with filters and what not, but I don't know it
    let mut similarity_score = 0;
    for entry in data0 {
        similarity_score += entry * data1.iter().filter(|d| **d==entry).count() as i32;
    }
    println!("The similarity score is {}", similarity_score);
    // 18805872

    Ok(())
}