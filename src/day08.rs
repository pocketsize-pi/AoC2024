use std::collections::{HashMap, VecDeque};
use aoc_2024::*;

pub fn day08(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 08");


    let data = read_input(08, input_type, manual_name)?;

    let max_width = data[0][0].len();
    let max_height = data.len();
    let mut antennas = HashMap::new();

    for (r, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        let char_row = str_to_chars(&row[0]);
        for (c, col) in char_row.iter().enumerate() {
            if *col != '.' {
                // actual antenna
                let point = Point{r_y:r as i32, c_x: c as i32};
                if !antennas.contains_key(col) {
                    let mut antenna_start_list = VecDeque::new();
                    antenna_start_list.push_back(point);
                    antennas.insert(*col, antenna_start_list);
                }
                else {
                    let values = antennas.get_mut(col).unwrap();
                    values.push_back(point);
                }
            }
        }
    }

    // println!("antennas: {:?}", antennas);

    fn find_antinodes (pt1: Point, pt2: Point) -> [Point; 2] {
        let dy = pt2.r_y - pt1.r_y;
        let dx = pt2.c_x - pt1.c_x;
        [Point{r_y: pt1.r_y - dy, c_x: pt1.c_x - dx}, Point{r_y: pt2.r_y + dy, c_x: pt2.c_x + dx}]
    }


    let mut antinodes_list = Vec::new();
    for antenna_values in antennas.values(){
        if antenna_values.len() == 1 {
            // a single antenna cannot create an antinode
            break;
        }
        else {
            let mut working_values = antenna_values.clone();

            while working_values.len() > 0 {
                let antenna1 = working_values.pop_front().unwrap();
                for id2 in 0..working_values.len() {
                    let antenna2 = working_values[id2];
                    // println!("a1: {:?}, a2: {:?}", antenna1, antenna2);
                    let antinodes = find_antinodes(antenna1, antenna2);
                    // println!("Antinodes: {:?}", antinodes);

                    for antinode in antinodes {
                        if antinode.within_dimensions(max_height as i32, max_width as i32) {
                            if !antinodes_list.contains(&antinode) {
                                antinodes_list.push(antinode);
                            }
                        }
                    }

                }
            }

        }
    }

    println!("There are {} antinodes", antinodes_list.len());
    // 249 antinodes


    Ok(())
}