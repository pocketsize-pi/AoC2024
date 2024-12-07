use aoc_2024::*;

pub fn day06(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 06");


    let data = read_input(06, input_type, manual_name)?;
    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut walls : Vec<Point> = Vec::new();
    let mut guard = OrientedPoint::default();


    for (r_i, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        for (c_i, col) in str_to_chars(&row[0]).iter().enumerate() {
            if *col == '#' {
                // is a wall!
                walls.push(Point{r_y: r_i as i32, c_x: c_i as i32});
            }
            else if *col == '^' {
                // guard starts looking up, I checked!
                guard.point.c_x = c_i as i32;
                guard.point.r_y = r_i as i32;
                guard.orientation = Direction::North;
            }
        }
        //
        let mut grid_row = str_to_chars(&row[0]);
        // guard starts looking up, I checked!
        if row[0].contains('^') {
            let guard_index = row[0].find('^').unwrap();
            guard.point.c_x = guard_index as i32;
            guard.point.r_y = r_i as i32;
            guard.orientation = Direction::North;
            grid_row[guard_index] = '.';
        }
        grid.push(grid_row);

    }

    let max_width = grid[0].len() as i32;
    let max_height = grid.len() as i32;

    let mut visited_points = 1;
    let mut visited_history = vec!(guard.point);

    let original_guard = guard;

    loop {
        let mut target_guard = guard;
        // println!("{:?}", target_guard);
        target_guard.move_one();
        if (target_guard.point.c_x == max_width) | (target_guard.point.r_y == max_height) |
            (target_guard.point.c_x == -1) | (target_guard.point.r_y == -1) {
            break;
            // not sure why the while wasn't working!
        }
        if walls.contains(&target_guard.point) {
            // rotate right
            guard.rotate(&Turning::Right);
        } else {
            // we can go to target pos!
            guard = target_guard;
            if !visited_history.contains(&guard.point) {
                visited_points += 1;
                visited_history.push(guard.point);
            }
        }
    }

    println!("The guard patrols {} points", visited_points);
    // let debug_point = Point{c_x: 3, r_y: 6};

    let mut loops_found = 0;
    // visited_history = Vec::new();
    for r in 0..max_height {
        for c in 0..max_width {

            let obstacle = Point {c_x: c, r_y: r};
            if !walls.contains(&obstacle) & (obstacle != original_guard.point) {
                let mut new_walls = walls.clone();
                new_walls.push(obstacle);
                // println!("\n{:?}", obstacle);

                // now we go again over, but with some modifications

                // let mut loop_found = false;
                // the key to a loop is the guard on an identical position, including orientation
                let mut oriented_history = Vec::new();

                // initialise history
                oriented_history.push(original_guard);

                visited_history = Vec::new();
                visited_history.push(original_guard.point);
                // visited_points = 1;

                guard = original_guard;

                loop {
                    let mut target_guard = guard;
                    // println!("{:?}", target_guard);
                    target_guard.move_one();
                    if (target_guard.point.c_x == max_width) | (target_guard.point.r_y == max_height) |
                        (target_guard.point.c_x == -1) | (target_guard.point.r_y == -1) {
                        // println!("exit");
                        break;
                        // not sure why the while wasn't working!
                    }
                    if new_walls.contains(&target_guard.point) {
                        // rotate right
                        guard.rotate(&Turning::Right);
                    } else {
                        // we can go to target pos!
                        guard = target_guard;

                        if !oriented_history.contains(&guard) {
                            // visited_points += 1;
                            oriented_history.push(guard);
                            visited_history.push(guard.point);
                        }
                        else {
                            // println!("found a loop");
                            // oriented guard means a loop!
                            loops_found += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("There are {} positions for the obstacle", loops_found);
    // 1686, first time running too!

    Ok(())
}