use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
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



    for char_instr in &instructions {

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

    let mut gps_score = 0;
    for (pt, thing) in full_map {
        if thing == BOX {
            gps_score += (100 * pt.r_y) + pt.c_x;
        }
    }

    println!("The final GPS coordinate is {}", gps_score);
    // 1497888 correct



    ///////////////////////////////////////////////////////////////
    let mut wide_walls = Vec::new();
    // let mut boxes = Vec::new();
    robot= Point::default();
    let mut wide_full_map = HashMap::new();
    // let mut instructions : Vec<char> = Vec::new();

    let wide_width = 2 * str_to_chars(&data[0][0]).len() as i32;

    const BOX1: char = '[';
    const BOX2: char = ']';

    for (r, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        if row.is_empty() {
            break;
        }
        else {

            let row_str = str_to_chars(&row[0]);
            for (c, col) in row_str.iter().enumerate() {
                let pt1 = Point {r_y: r as i32, c_x: 2*c as i32 };
                let pt2 = Point {r_y: r as i32, c_x: 1 + 2*c as i32 };
                if *col == WALL {
                    wide_walls.push(pt1);
                    wide_walls.push(pt2);
                    wide_full_map.insert(pt1, WALL);
                    wide_full_map.insert(pt2, WALL);
                }
                else if *col == BOX {
                    // boxes.push(pt);
                    wide_full_map.insert(pt1, BOX1);
                    wide_full_map.insert(pt2, BOX2);
                }
                else if *col == ROBOT {
                    robot = pt1;
                    // full_map.insert(pt, *col);
                }
            }
        }

    }

    for r in 0..height {
        for c in 0..wide_width {
            let pt = Point {r_y: r, c_x: c};
            if pt == robot {
                print!("@");
            }
            else if wide_full_map.contains_key(&pt) {
                print!("{}", wide_full_map.get(&pt).unwrap());
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();


    fn find_half_box (box_loc: &Point, full_map: &HashMap<Point, char>) -> Point {
        if *full_map.get(box_loc).unwrap() == BOX1 {
            Point{r_y: box_loc.r_y, c_x: box_loc.c_x+1}
        }
        else {
            Point{r_y: box_loc.r_y, c_x: box_loc.c_x-1}
        }
    }

    for char_instr in &instructions {

        // println!();
        // println!("{char_instr}");

        let instr = get_arrow_dir(&char_instr);

        let mut new_robot = robot;
        new_robot.move_one_udlr(&instr);
        if wide_full_map.contains_key(&new_robot) {




            let here = wide_full_map.get(&new_robot).unwrap();
            if *here != WALL {

                if (instr == UdlrDirection::Right) | (instr == UdlrDirection::Left) {
                    // if something that is not a wall, then we get to move the thing
                    // check how many boxes we have to move
                    let mut box_list = Vec::new();
                    let mut box_loc = new_robot;
                    let mut can_move = true;
                    while wide_full_map.contains_key(&box_loc) {
                        let new_here = wide_full_map.get(&box_loc).unwrap();
                        if *new_here != WALL {
                            box_list.push((box_loc, *new_here));
                            box_loc.move_one_udlr(&instr);
                        } else {
                            can_move = false;
                            break;
                        }
                    }
                    // ok, let's see if we can move, and how many boxes
                    if can_move {
                        // println!("moving {} boxes", box_list.len());
                        // box_loc = new_robot;
                        robot = new_robot;
                        for box_loc in &box_list {
                            wide_full_map.remove(&box_loc.0).unwrap();
                        }
                        for box_loc in &box_list {
                            let mut new_loc = box_loc.0;
                            new_loc.move_one_udlr(&instr);
                            wide_full_map.insert(new_loc,box_loc.1);
                        }
                    }
                }
                else {
                    // up or down move diagonal boxes!

                    let mut box_list = Vec::new();
                    let mut box_loc_start = new_robot;
                    let mut can_move = true;

                    let mut box_queue = VecDeque::new();
                    let mut box_visited = Vec::new();

                    box_queue.push_back(box_loc_start);
                    box_queue.push_back(find_half_box(&box_loc_start, &wide_full_map));

                    while !box_queue.is_empty() {

                        let box_loc = box_queue.pop_front().unwrap();
                        box_visited.push(box_loc);

                        if wide_full_map.contains_key(&box_loc) {
                            let new_here = wide_full_map.get(&box_loc).unwrap();
                            // println!("checking {:?} {}", box_loc, new_here);
                            if *new_here != WALL {
                                box_list.push((box_loc, *new_here));
                                // add next box
                                let mut next_box = box_loc;
                                next_box.move_one_udlr(&instr);
                                if !box_queue.contains(&next_box) & !box_visited.contains(&next_box){
                                    box_queue.push_back(next_box);
                                }
                                // and it's sibling box, if not in yet
                                let half_box = find_half_box(&box_loc, &wide_full_map);
                                if !box_queue.contains(&half_box) & !box_visited.contains(&half_box){
                                    box_queue.push_back(half_box);
                                }
                            } else {
                                can_move = false;
                                break;
                            }
                        }
                    }


                    // ok, let's see if we can move, and how many boxes
                    if can_move {
                        // println!("moving {} boxes", box_list.len());
                        // box_loc = new_robot;
                        robot = new_robot;
                        for box_loc in &box_list {
                            wide_full_map.remove(&box_loc.0).unwrap();
                        }
                        for box_loc in &box_list {
                            let mut new_loc = box_loc.0;
                            new_loc.move_one_udlr(&instr);
                            wide_full_map.insert(new_loc,box_loc.1);
                        }
                    }


                }



            }
            // else wall and we do nothing
        } else {
            // this is a free space, we are ok
            robot = new_robot;
        }

        // for r in 0..height {
        //     for c in 0..wide_width {
        //         let pt = Point {r_y: r, c_x: c};
        //         if pt == robot {
        //             print!("@");
        //         }
        //         else if wide_full_map.contains_key(&pt) {
        //             print!("{}", wide_full_map.get(&pt).unwrap());
        //         }
        //         else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();
    }

    let mut wide_gps_score = 0;
    for (pt, thing) in wide_full_map {
        if thing == BOX1 {
            wide_gps_score += (100 * pt.r_y) + pt.c_x;
        }
    }

    println!("The final GPS coordinate is {}", wide_gps_score);
    // 1522420

    Ok(())
}