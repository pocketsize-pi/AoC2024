use aoc_2024::*;

pub fn day14(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 14");


    let data = read_input(14, input_type, manual_name)?;

    let width = if input_type == InputType::Data {101} else {11};
    let height = if input_type == InputType::Data {103} else {7};

    let mut fleet_pos = Vec::new();
    let mut fleet_vel = Vec::new();

    for row in &data {
        // println!("{:?}", row);
        let pos_str = &row[0].trim_start_matches("p=").split(',').collect::<Vec<&str>>();
        let vel_str = &row[1].trim_start_matches("v=").split(',').collect::<Vec<&str>>();

        fleet_pos.push(Point {c_x: str_to_i32(&pos_str[0]), r_y: str_to_i32(&pos_str[1])});
        fleet_vel.push(Point {c_x: str_to_i32(&vel_str[0]), r_y: str_to_i32(&vel_str[1])});
    }

    // for r in 0..height {
    //     for c in 0..width {
    //         let pt = Point {r_y: r, c_x: c};
    //         if fleet_pos.contains(&pt) {
    //             let num = fleet_pos.iter().filter(|robot| **robot == pt).count();
    //             print!("{num}");
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    // println!();


    let wait = 100;
    for _i in 0..wait {
        for robot_id in 0..fleet_pos.len() {
            let robot_pos = &mut fleet_pos[robot_id];
            let robot_vel = fleet_vel[robot_id];
            // println!("pos {:?}, vel {:?}", robot_pos, robot_vel);
            robot_pos.add(robot_vel);

            // check limits
            // ah! I forgot when it goes negative!
            if robot_pos.c_x >= width {
                robot_pos.c_x = robot_pos.c_x % width;
            }
            else if robot_pos.c_x < 0 {
                robot_pos.c_x = width - robot_pos.c_x.abs();
            }
            if robot_pos.r_y >= height {
                robot_pos.r_y = robot_pos.r_y % height;
            }
            else if robot_pos.r_y < 0 {
                robot_pos.r_y = height - robot_pos.r_y.abs();
            }
            // println!("new pos: {:?}", robot_pos);
            // I bet we can do this by multiplying by 100 and then just straight up %
        }

        // for r in 0..height {
        //     for c in 0..width {
        //         let pt = Point {r_y: r, c_x: c};
        //         if fleet_pos.contains(&pt) {
        //             let num = fleet_pos.iter().filter(|robot| **robot == pt).count();
        //             print!("{num}");
        //         }
        //         else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();
    }



    // now the quadrants
    let top_half = (height - 1) / 2;
    let bottom_half = height - top_half; // this should bypass the middle
    let left_half = (width - 1) / 2;
    let right_half = width - left_half;
    let top_left = fleet_pos.iter().filter(|robot| (robot.c_x < left_half) & (robot.r_y < top_half)).collect::<Vec<&Point>>().len();
    let top_right = fleet_pos.iter().filter(|robot| (robot.c_x >= right_half) & (robot.r_y < top_half)).collect::<Vec<&Point>>().len();
    let bottom_left = fleet_pos.iter().filter(|robot| (robot.c_x < left_half) & (robot.r_y >= bottom_half)).collect::<Vec<&Point>>().len();
    let bottom_right = fleet_pos.iter().filter(|robot| (robot.c_x >= right_half) & (robot.r_y >= bottom_half)).collect::<Vec<&Point>>().len();

    // println!("{} {} {} {}", top_half, bottom_half, left_half, right_half);
    // println!("{} {} {} {}", top_left, top_right, bottom_left, bottom_right);
    println!("Safety factor: {}", top_left* top_right* bottom_left* bottom_right);
    // 232589280 - it's nice when we get it at the first time running the full data

    // for r in 0..height {
    //     for c in 0..width {
    //         let pt = Point {r_y: r, c_x: c};
    //         if fleet_pos.contains(&pt) {
    //             let num = fleet_pos.iter().filter(|robot| **robot == pt).count();
    //             print!("{num}");
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    // println!();

    Ok(())
}