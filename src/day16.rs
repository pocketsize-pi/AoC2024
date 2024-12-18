use std::collections::VecDeque;
use aoc_2024::*;
use aoc_2024::Turning::{Left, Right};

pub fn day16(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 16");


    let data = read_input(16, input_type, manual_name)?;

    let mut maze = Vec::new();
    let mut reindeer_start = OrientedPoint::default();
    let mut end_location = Point::default();

    for (r, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        let row_str = str_to_chars(&row[0]);
        for (c, col) in row_str.iter().enumerate() {
            let pt = Point::new(c, r);
            if *col == WALL {
                maze.push(pt);
            }
            else if *col == 'S' {
                reindeer_start = OrientedPoint{ point: pt, orientation: Direction::East };
            }
            else if *col == 'E' {
                end_location = pt;
            }

        }
    }

    // println!("Reindeer starts at {:?}, end is {:?}", reindeer_start, end_location);

    let move_score = 1;
    let turn_score = 1000;

    enum Choices {
        MoveAlong,
        RotateRight,
        RotateLeft,
    }

    let list_options = [Choices::MoveAlong, Choices::RotateLeft, Choices::RotateRight];

    // two synced queues
    let mut steps_queue = VecDeque::new();
    let mut score_queue = VecDeque::new();
    // let mut joint_queue = VecDeque::new();
    let mut steps_history = Vec::new();
    let mut score_history = Vec::new();

    steps_queue.push_back(reindeer_start);
    score_queue.push_back(0);


    let mut end_reached = false;

    while !steps_queue.is_empty() {

        let current_reindeer = steps_queue.pop_front().unwrap();
        let current_score = score_queue.pop_front().unwrap();
        steps_history.push(current_reindeer);
        score_history.push(current_score);
        // println!("current loc: {:?}, score: {}", current_reindeer, current_score);

        for choice in &list_options {
            // perform movement using a match

            let mut new_reindeer = current_reindeer;
            let mut new_score = current_score;

            match choice {
                Choices::MoveAlong => {
                    new_reindeer.move_one();
                    new_score += move_score;
                }
                Choices::RotateRight => {
                    new_reindeer.rotate(&Right);
                    new_score += turn_score;
                }
                Choices::RotateLeft => {
                    new_reindeer.rotate(&Left);
                    new_score += turn_score;
                }
            }

            // now check if we are done
            if new_reindeer.point == end_location {
                // this will only happen after a move along
                println!("Ended with a final score of {}", new_score);
                end_reached = true;
            }
            else if !maze.contains(&new_reindeer.point) {
                // this is a new legal location
                if !steps_history.contains(&new_reindeer) & !steps_queue.contains(&new_reindeer){
                    // they really should be sorted by score
                    // this finds the value of the first entry that is bigger than ours, so we want to go before
                    let find_index = score_queue.iter().position(|x| *x > new_score);
                    match find_index {
                        None => {
                            // none bigger, goes at the end
                            score_queue.push_back(new_score);
                            steps_queue.push_back(new_reindeer);
                        }
                        Some(id) => {
                            if id == 0 {
                                // the first one is bigger, so we insert at the front directly
                                score_queue.push_front(new_score);
                                steps_queue.push_front(new_reindeer);
                            }
                            else {
                                // we stick it in id-1
                                score_queue.insert(id-1, new_score);
                                steps_queue.insert(id-1, new_reindeer);
                            }
                        }
                    }


                }
            }
        }

        // sort by score

        if end_reached {break}

    }

    // oh dear, my answer 79412 is too high!

    Ok(())
}