use std::collections::HashMap;
use aoc_2024::*;

pub fn day10(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 10");


    let data = read_input(10, input_type, manual_name)?;
    let mut topological_map = HashMap::new();
    let mut start_positions = Vec::new();
    for (r, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        let chars = str_to_chars(&row[0]);
        for (c, col) in chars.iter().enumerate() {
            let val = col.to_digit(10).unwrap();
            topological_map.insert(Point{r_y: r as i32, c_x: c as i32 }, val);
            if val == 0 {
                start_positions.push(Point{r_y: r as i32, c_x: c as i32 });
            }
        }
    }

    let mut num_trails = 0;
    let mut trail_score = 0;

    let trail_dirs = [Direction::North, Direction::East, Direction::South, Direction::West];

    for start_pos in start_positions {
        let mut trail_queue = vec![start_pos];
        let mut visited_positions = Vec::new();
        num_trails = 0;

        while !trail_queue.is_empty() {
            let current_pos = trail_queue.pop().unwrap();
            visited_positions.push(current_pos);

            let current_slope = topological_map.get(&current_pos).unwrap();

            // find all adjacent points
            for dir in trail_dirs {
                let mut new_point = current_pos;
                new_point.move_one(&dir);
                if topological_map.contains_key(&new_point) {
                    let new_slope = topological_map.get(&new_point).unwrap();
                    if *new_slope == (current_slope + 1) {
                        // good point, but have we see it yet?
                        if !visited_positions.contains(&new_point) {
                            if *new_slope == 9 {
                                num_trails += 1;
                            }
                            // we should add it regardless, so that we don't check the 9s again
                            trail_queue.push(new_point);
                        }
                    }
                }
            }
        }
        // end of a trail head, let's add the score
        // println!("Current trails: {}", num_trails);
        trail_score +=  num_trails;
    }
    println!("There total score is {}", trail_score );
    // 574 first try


    Ok(())
}