use aoc_2024::*;

pub fn day02(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 02");


    let data = read_input(02, input_type, manual_name)?;
    let mut report_master = Vec::new();
    for row in &data {
        // println!("{:?}", row);
        // using i32 to make the calculations later easier
        let numbers : Vec<i32> = row.iter().map(|s| s.parse().unwrap()).collect();
        // println!("{:?}", numbers);
        report_master.push(numbers);
    }

    fn direction(r0: i32, r1: i32) -> i32 {
        let sign: i32;
        if r0 > r1 {sign = 1;}
        else if r0 < r1 {sign = -1;}
        else {sign = 0;}
        sign
    }
    fn within_limits(r0: i32, r1: i32) -> bool {
        ((r0 - r1).abs() <= 3) & ((r0 - r1).abs() >=1)
    }

    let mut total_safe = 0;

    let mut unsafe_reports = Vec::new();

    for report in &report_master {
        let level_dir = direction(report[0], report[1]);
        let mut report_valid = true;
        for l in 0..report.len()-1 {
            if (direction(report[l], report[l+1]) != level_dir) |
                !within_limits(report[l], report[l+1]) {
                report_valid = false;
                break;
            }
        }
        if report_valid {
            total_safe += 1;
        }
        else {
            unsafe_reports.push(report.clone());
        }
    }
    println!("There are {} safe reports", total_safe);
    // 356

    let mut dampen_total_safe = 0;
    // brute force!
    for report in &unsafe_reports {
        let mut dampen_report_valid = false;
        for r in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(r);

            let level_dir = direction(new_report[0], new_report[1]);
            let mut report_valid = true;
            for l in 0..new_report.len()-1 {
                if (direction(new_report[l], new_report[l+1]) != level_dir) |
                    !within_limits(new_report[l], new_report[l+1]) {
                    report_valid = false;
                    break;
                }
            }

            if report_valid {
                dampen_report_valid = true;
                break;
            }
        }
        if dampen_report_valid {
            dampen_total_safe += 1;
        }
    }

    println!("There are {} safe reports with dampening", total_safe + dampen_total_safe);
    // 414 is too high
    // 375 too low, so is 379
    // 413 - brute force saves the day. Also, I was so very close!

    Ok(())
}