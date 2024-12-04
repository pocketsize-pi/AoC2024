use aoc_2024::*;
use regex::Regex;


pub fn day03(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 03");


    let data = read_input(03, input_type, manual_name)?;

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut values = vec![];
    let mut final_score = 0;
    for row in &data {
        for cell in row {
            for (_, [f1, f2]) in re.captures_iter(&*cell).map(|c| c.extract()) {
                values.push((f1.parse::<i32>()?, f2.parse::<i32>()?));
                final_score += f1.parse::<i32>()? * f2.parse::<i32>()?;
            }
        }
    }


    // println!("{:?}", values);
    println!("Final value is: {}", final_score);
    // 843172 too low (because I hadn't used all the input data)
    // 9499483 also too low - still not using all the data! there are spaces, because of my input handling
    // 170778545 finally!

    //let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").map(|(i, _)|i).collect();
    // assert_eq!(v, [0,6,12]);

    let mut mult_enabled = true;
    final_score = 0;
    for row in &data {
        for cell in row {
            // find do & don't indeces
            // println!("cell: {:?}", cell);
            let mut do_i : Vec<_>= cell.match_indices("do()").map(|(i, _)|i).collect();
            let mut dont_i : Vec<_>= cell.match_indices("don't()").map(|(i, _)|i).collect();
            for (ft, [f1, f2]) in re.captures_iter(&*cell).map(|c| c.extract()) {

                // let values = (f1.parse::<i32>()?, f2.parse::<i32>()?);
                // println!("\ndo_i {:?}", do_i);
                // println!("dont_i {:?}", dont_i);
                // println!("ft {:?}", ft);

                // find index of value
                // I'm assuming there are no repeated ones in the same row
                let ft_i : Vec<_> = cell.match_indices(ft).map(|(i, _)|i).collect();

                if ft_i.len()>1 {
                    println!("multiples!");
                }

                // println!("ft_i {:?}", ft_i);
                if !do_i.is_empty() & !dont_i.is_empty() {
                    if (ft_i[0] > do_i[0]) & (ft_i[0] > dont_i[0]) {
                        // println!("This is the issue");
                    }


                    if (ft_i[0] > do_i[0]) & (ft_i[0] > dont_i[0]) {
                        // println!("Issue!!!!!");
                    }
                    else if ft_i[0] > do_i[0] {
                        do_i.pop();
                        mult_enabled = true;
                        // println!("enable1");
                    }
                    else if ft_i[0] > dont_i[0] {
                        dont_i.pop();
                        mult_enabled = false;
                        // println!("disable1");
                    }

                }
                else if !do_i.is_empty() & dont_i.is_empty() {
                    if ft_i[0] > do_i[0] {
                        do_i.pop();
                        mult_enabled = true;
                        // println!("enable2");
                    }
                }
                else if !dont_i.is_empty() & do_i.is_empty() {
                    if ft_i[0] > dont_i[0] {
                        dont_i.pop();
                        mult_enabled = false;
                        // println!("disable2");
                    }
                }

                if mult_enabled {
                    // println!("mult");
                    final_score += f1.parse::<i32>()? * f2.parse::<i32>()?;
                }

            }
            if !do_i.is_empty() | !dont_i.is_empty() {
                // process what we have left
                // println!("leftovers!");
                // println!("do_i {:?}", do_i);
                // println!("dont_i {:?}", dont_i);
                if !do_i.is_empty() {
                    do_i.pop();
                    mult_enabled = true;
                    // println!("enable3");
                }
                else {
                    dont_i.pop();
                    mult_enabled = false;
                    // println!("disable3");
                }
            }
        }
    }

    println!("Final value is: {}", final_score);
    // 86719901 too high
    // 80992309 too low
    // 82859732 still loo low
    // 82868252 yessss!

    Ok(())
}