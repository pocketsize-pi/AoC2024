use std::collections::HashMap;
use aoc_2024::*;

pub fn day12(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 12");


    let data = read_input(12, input_type, manual_name)?;

    let mut plot_map = HashMap::new();

    for (r, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        let row_char = str_to_chars(&row[0]);
        for (c, col) in row_char.iter().enumerate() {
            let point = Point{r_y: r as i32, c_x: c as i32};
            if plot_map.contains_key(col) {
                let entry : &mut Vec<Point> = plot_map.get_mut(col).unwrap();
                entry.push(point);
            }
            else {
                plot_map.insert(*col, vec!(point));
            }
        }
    }

    let trail_dirs = [Direction::North, Direction::East, Direction::South, Direction::West];

    let mut total_cost = 0;
    for crop in plot_map.keys() {

        let crop_locations = plot_map.get(crop).unwrap().clone();
        let mut location_list = plot_map.get(crop).unwrap().clone();

        // println!("\nCrop: {}, over {:?}", crop, crop_locations);
        let mut visited_locs = Vec::new();
        let mut loc_queue = vec!(location_list[0]);

        while !location_list.is_empty() {
            let mut current_perimeter = 0;
            let mut current_area = 0;
            while !loc_queue.is_empty() {
                let current_loc = loc_queue.pop().unwrap();
                // remove from master list
                // println!("{:?}", current_loc);
                location_list.retain(|&x| x != current_loc);

                visited_locs.push(current_loc);
                current_area += 1;

                // find nearby points
                let mut num_dirs = 0;
                for dir in trail_dirs {
                    let mut new_loc = current_loc;
                    new_loc.move_one(&dir);
                    if crop_locations.contains(&new_loc) {
                        num_dirs += 1;
                        if !visited_locs.contains(&new_loc) & !loc_queue.contains(&new_loc){
                            loc_queue.push(new_loc);
                        }
                    }
                }
                current_perimeter += (4 - num_dirs);

            }
            // println!("current cost of {}: ({}) * ({}) = {}", crop, current_perimeter, current_area, current_perimeter * current_area);
            total_cost += (current_perimeter * current_area);

            // loop?
            if !location_list.is_empty() {
                loc_queue.push(location_list[0]);
            }
        }

    }

    println!("Final cost is: {}", total_cost);
    // 1450816 is the right answer

    // we had something similar last year, I had to copy someone else's maths, I'm going to try later

    Ok(())
}