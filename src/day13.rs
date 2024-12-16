use std::collections::VecDeque;
use aoc_2024::*;

pub fn day13(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 13");


    let data = read_input(13, input_type, manual_name)?;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct Machine {
        button_a: Point,
        button_b: Point,
        prize: Point,
    }

    let mut arcade = Vec::new();
    let mut r = 0;
    while r < data.len() {
        // println!("{:?}", row);
        let row_a = &data[r];
        let row_b = &data[r+1];
        let row_prize = &data[r+2];

        let a_x = str_to_i32(row_a[2].trim_end_matches(',').trim_start_matches("X+"));
        let a_y = str_to_i32(row_a[3].trim_start_matches("Y+"));

        let b_x = str_to_i32(row_b[2].trim_end_matches(',').trim_start_matches("X+"));
        let b_y = str_to_i32(row_b[3].trim_start_matches("Y+"));

        let p_x = str_to_i32(row_prize[1].trim_end_matches(',').trim_start_matches("X="));
        let p_y = str_to_i32(row_prize[2].trim_start_matches("Y="));

        // println!("{:?}", row_a);
        // println!("{:?}", a_x);

        let machine = Machine {
            button_a: Point{c_x: a_x, r_y: a_y},
            button_b: Point{c_x: b_x, r_y: b_y},
            prize: Point{c_x: p_x, r_y: p_y},
        };

        arcade.push(machine);

        r += 4;
    }

    // println!("{:?}", arcade);


    let mut total_tokens: i32 = 0;

    enum Buttons {
        A, B,
    }
    let button_order = [Buttons::A, Buttons::B];

    let starting_point = Point::default();

    let cost_a: i32 = 3;
    let cost_b: i32 = 1;

    #[derive(Clone, Copy, PartialEq)]
    struct MachineStatus {
        claw: Point,
        cost: i32,
        num_a: i32,
        num_b: i32,
    }



    for machine in &arcade {

        // println!("Machine: {:?}", machine);

        let mut button_queue = VecDeque::new();

        button_queue.push_back(MachineStatus{claw: starting_point, cost: 0, num_a: 0, num_b: 0});
        let mut found = false;
        while !button_queue.is_empty() {
            let current_status = button_queue.pop_front().unwrap();

            // println!("dumb print: {:?}", current_status.claw);

            for next_button in &button_order {
                let mut move_claw = current_status;
                match next_button {
                    Buttons::A => {
                        move_claw.claw.add(machine.button_a);
                        move_claw.cost += cost_a;
                        move_claw.num_a += 1;
                    }
                    Buttons::B => {
                        move_claw.claw.add(machine.button_b);
                        move_claw.cost += cost_b;
                        move_claw.num_b += 1;
                    }
                }

                if move_claw.claw == machine.prize {
                    // println!("Found at cost {}", move_claw.cost);
                    total_tokens += move_claw.cost;
                    found = true;
                    break;
                }
                else {
                    // check we haven't overshot
                    if (move_claw.claw.c_x < machine.prize.c_x) & (move_claw.claw.r_y < machine.prize.r_y) {
                        // add to queue if not in already
                        // the order of button pushing doesn't matter
                        // we'll get to the same point
                        if !button_queue.contains(&move_claw)
                        {
                            button_queue.push_back(move_claw);
                        }

                    }
                }

            }

            if found {break}
        }
        // if !found {
        //     println!("This machine can't be won");
        // }

    }

    println!("Final cost is: {}", total_tokens);
    // 35729

    // let's solve the equations
    // [ a_x b_x ] [ A ] = [ p_x ]
    // [ a_y b_y ] [ B ] = [ p_y ]

    let mut big_tokens = 0;
    let crazy = 10000000000000.0;
    for machine in &arcade {

        // println!("Machine: {:?}", machine);
        let determinant = ((machine.button_a.c_x * machine.button_b.r_y) - (machine.button_b.c_x * machine.button_a.r_y)) as f64;

        // inverse
        // 1 /det [  b_y -b_x ]
        //        [ -a_y  a_x ]

        // sol = inv * p
        // a = 1/det * ( (by * px) + (-bx * py)
        // println!("determinant: {}", determinant);
        if determinant != 0.0 {
            let a_y = machine.button_a.r_y as f64;
            let b_y = machine.button_b.r_y as f64;
            let a_x = machine.button_a.c_x as f64;
            let b_x = machine.button_b.c_x as f64;
            let p_y = machine.prize.r_y as f64 + crazy;
            let p_x = machine.prize.c_x as f64 + crazy;

            let num_a = ((b_y * p_x) - (b_x * p_y)) / determinant;
            let num_b = ((-a_y * p_x) + (a_x * p_y)) / determinant;

            if (num_a % 1.0 == 0.0) & (num_b % 1.0 == 0.0) {
                let current_cost = (num_a as i64 * cost_a as i64) + (num_b as i64 * cost_b as i64);
                // println!("Current cost: {}", current_cost);
                big_tokens += current_cost;
            }

            // println!("num {} {}", num_a, ((machine.button_b.r_y * machine.prize.c_x) - (machine.button_b.c_x * machine.prize.r_y)));
            // println!("num {} {}", num_b, ((-machine.button_a.r_y * machine.prize.c_x) + (machine.button_a.c_x * machine.prize.r_y)));


        }


    }
    println!("Final cost is: {}", big_tokens);
    //158299988799973 too high
    // 88584689879723 is right!

    Ok(())
}