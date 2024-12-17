use std::collections::HashMap;
use aoc_2024::*;

pub fn day15(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 15");


    let data = read_input(15, input_type, manual_name)?;

    let mut walls = Vec::new();
    let mut boxes = Vec::new();
    let mut robot= Point::default();
    let mut full_map = HashMap::new();
    let mut instructions : Vec<char> = Vec::new();

    const WALL : char = '#';
    const BOX : char = 'O';
    const ROBOT : char = '@';

    let width = str_to_chars(&data[0][0]).len() as i32;
    let mut height: i32 = 0;

    let mut process_map = true;
    for (r, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        if row.is_empty() {
            process_map = false;
            height = r as i32;
        }
        else {

            if process_map {
                let row_str = str_to_chars(&row[0]);
                for (c, col) in row_str.iter().enumerate() {
                    let pt = Point {r_y: r as i32, c_x: c as i32 };
                    if *col == WALL {
                        walls.push(pt);
                        full_map.insert(pt, *col);
                    }
                    else if *col == BOX {
                        boxes.push(pt);
                        full_map.insert(pt, *col);
                    }
                    else if *col == ROBOT {
                        robot = pt;
                        // full_map.insert(pt, *col);
                    }
                }
            }
            else {

                // instructions
                let mut instr = str_to_chars(&row[0]);
                instructions.append(&mut instr);
            }

        }

    }

    fn get_arrow_dir(c: &char) -> UdlrDirection {
        if c == &'^' {
            UdlrDirection::Up
        }
        else if c == &'v' {
            UdlrDirection::Down
        }
        else if c == &'<' {
            UdlrDirection::Left
        }
        else { //>
            UdlrDirection::Right
        }
    }

    fn update_object (box_loc: Point, instr: UdlrDirection, full_map: &mut HashMap<Point, char>) {
        full_map.remove(&box_loc);
        let mut new_box_loc = box_loc;
        new_box_loc.move_one_udlr(&instr);
        full_map.insert(new_box_loc, BOX);
    }

    for char_instr in instructions {

        // println!();
        // println!("{char_instr}");

        let instr = get_arrow_dir(&char_instr);

        let mut new_robot = robot;
        new_robot.move_one_udlr(&instr);
        if full_map.contains_key(&new_robot) {
            let here = full_map.get(&new_robot).unwrap();
            if *here != WALL {
                // if something that is not a wall, then we get to move the thing
                // check how many boxes we have to move
                let mut box_count = 0;
                let mut box_loc = new_robot;
                let mut can_move = true;
                while full_map.contains_key(&box_loc) {
                    let new_here = full_map.get(&box_loc).unwrap();
                    if *new_here != WALL {
                        box_count += 1;
                        box_loc.move_one_udlr(&instr);
                    }
                    else {
                        can_move = false;
                        break;
                    }
                }
                // ok, let's see if we can move, and how many boxes
                if can_move {
                    // println!("moving {box_count} boxes");
                    box_loc = new_robot;
                    robot = new_robot;
                    full_map.remove(&box_loc);
                    box_loc.move_along_udlr(&instr, &box_count);
                    full_map.insert(box_loc, BOX);
                    // for _b in 0..box_count {
                    //     update_object(box_loc, instr, &mut full_map);
                    //     box_loc.move_one_udlr(&instr);
                    // }
                }
            }
            // else wall and we do nothing
        }
        else {
            // this is a free space, we are ok
            robot = new_robot;
        }


        // printout

        // for r in 0..height {
        //     for c in 0..width {
        //         let pt = Point {r_y: r, c_x: c};
        //         if pt == robot {
        //             print!("@");
        //         }
        //         else if full_map.contains_key(&pt) {
        //             print!("{}", full_map.get(&pt).unwrap());
        //         }
        //         else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();

    }

    for r in 0..height {
        for c in 0..width {
            let pt = Point {r_y: r, c_x: c};
            if pt == robot {
                print!("@");
            }
            else if full_map.contains_key(&pt) {
                print!("{}", full_map.get(&pt).unwrap());
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    let mut gps_score = 0;
    for (pt, thing) in full_map {
        if thing == BOX {
            gps_score += (100 * pt.r_y) + pt.c_x;
        }
    }

    println!("The final GPS coordinate is {}", gps_score);
    // 1497888 correct


    Ok(())
}