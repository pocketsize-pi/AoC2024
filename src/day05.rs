use aoc_2024::*;

pub fn day05(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 05");


    let data = read_input(05, input_type, manual_name)?;

    let mut page_order : Vec<(u32, u32)> = Vec::new();
    let mut print_manuals : Vec<Vec<u32>> = Vec::new();
    let mut do_page_order = true;
    for row in &data {
        // println!("{:?}", row);
        if do_page_order {
            if row.is_empty() {
                do_page_order = false;
            }
            else {
                // process page order
                let order : Vec<&str>= row[0].split("|").collect::<Vec<&str>>();
                page_order.push((str_to_u32(&order[0]), str_to_u32(&order[1])));
            }
        }
        else {
            // manual pages
            let pages : Vec<&str>= row[0].split(",").collect::<Vec<&str>>();
            // let nums = lines[0].chars().map(|c| c.to_digit(RADIX).expect("conversion error"));
            let pages_num : Vec<u32> = pages.iter().map(|num| str_to_u32(num)).collect();
            print_manuals.push(pages_num);
        }
    }
    // println!("{:?}", page_order);
    // println!("{:?}", print_manuals);

    let mut middle_pages_sum = 0;
    for manual in print_manuals {
        let mut is_ordered = true;
        for order in &page_order {
            if manual.contains(&order.0) & manual.contains(&order.1) {
                // check
                let first_i = manual.iter().position(|&x| x == order.0).unwrap();
                let second_i = manual.iter().position(|&x| x == order.1).unwrap();
                if first_i > second_i {
                    is_ordered = false;
                    break;
                }
            }
        }
        if is_ordered {
            // now we need to find the middle point
            //assuming odd length
            let mid_point = (manual.len()-1)/2;
            middle_pages_sum += manual[mid_point];
        }
    }
    println!("Middle pages sum is: {}", middle_pages_sum);
    // 5329

    Ok(())
}