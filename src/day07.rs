use std::collections::VecDeque;
use aoc_2024::*;

pub fn day07(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 07");


    let data = read_input(07, input_type, manual_name)?;

    #[derive(Debug)]
    struct Calculation {
        result: u64,
        factors: VecDeque<u64>,
    }

    let mut calculation_list = Vec::new();

    for row in &data {
        // println!("{:?}", row);

        let result = str_to_u64(row[0].trim_matches(':'));
        let mut factors = VecDeque::new();
        for f in 1..row.len() {
            factors.push_back(str_to_u64(&row[f]));
        }
        calculation_list.push(Calculation{result, factors});
    }
    //
    // enum Operations {
    //     Add,
    //     Multiply,
    // }
    //

    fn try_operation(expected: u64, current_sum: u64, factors: &VecDeque<u64>) -> bool{

        if factors.len() == 1 {
            // last/only pair, so let's compare
            let try_sum_result = current_sum + factors[0];
            let try_mult_result = current_sum * factors[0];
            // println!("last instance total: {current_sum}; next sum: {try_sum_result}, next mult: {try_mult_result}");
            if (expected == try_sum_result) | (expected == try_mult_result){
                // hurrah!
                return true
            }
            else {
                return false
            }
        }
        else {
            // try current then go down
            // add

            let try_sum_result =  current_sum + factors[0];
            let try_mult_result =  current_sum * factors[0];
            // println!("current total: {current_sum}; next sum: {try_sum_result}, next mult: {try_mult_result}");
            let mut down_factors = factors.clone();
            down_factors.pop_front();
            let down_sum : bool;
            let down_mult : bool;
            if try_sum_result <= expected {
                down_sum = try_operation(expected, try_sum_result, &down_factors);
            }
            else { down_sum = false }
            if try_mult_result <= expected {
                down_mult = try_operation(expected, try_mult_result, &down_factors);
            }
            else { down_mult = false }
            return down_sum | down_mult

        }

    }

    let mut total_calibration = 0;
    for calculation in &calculation_list {
        // println!("\n{:?}", calculation);
        if try_operation(calculation.result, 0, &calculation.factors) {
            // is good, let's add it
            // println!("is valid");
            total_calibration += calculation.result;
        }
    }

    println!("Total calibration value is {}", total_calibration);
    // 4019885351117 too low
    // 4122618559853 was right! Less _or equal_ work for mult if the last one is one

    fn try_triple_operation(expected: u64, current_sum: u64, factors: &VecDeque<u64>) -> bool{

        if factors.len() == 1 {
            // last/only pair, so let's compare
            let try_sum_result = current_sum + factors[0];
            let try_mult_result = current_sum * factors[0];
            let mut concat : String = current_sum.to_string().to_owned();
            concat.push_str(&*factors[0].to_string());
            let try_concat_result : u64 = str_to_u64(&concat);
            // println!("last instance total: {current_sum}; next sum: {try_sum_result}, next concat: {try_mult_result}, next mult: {try_concat_result}");
            if (expected == try_sum_result) | (expected == try_mult_result) | (expected ==  try_concat_result){
                // hurrah!
                return true
            }
            else {
                return false
            }
        }
        else {
            // try current then go down
            // add

            let try_sum_result =  current_sum + factors[0];
            let try_mult_result =  current_sum * factors[0];
            let mut concat : String = current_sum.to_string().to_owned();
            concat.push_str(&*factors[0].to_string());
            let try_concat_result : u64 = str_to_u64(&concat);
            // println!("current total: {current_sum}; next sum: {try_sum_result}, next concat: {try_mult_result}, next mult: {try_concat_result}");
            let mut down_factors = factors.clone();
            down_factors.pop_front();
            let down_sum : bool;
            let down_mult : bool;
            let down_concat : bool;
            if try_sum_result <= expected {
                down_sum = try_triple_operation(expected, try_sum_result, &down_factors);
            }
            else { down_sum = false }
            if try_mult_result <= expected {
                down_mult = try_triple_operation(expected, try_mult_result, &down_factors);
            }
            else { down_mult = false }
            if try_concat_result <= expected {
                down_concat = try_triple_operation(expected, try_concat_result, &down_factors);
            }
            else { down_concat = false }
            return down_sum | down_mult | down_concat

        }

    }

    let mut total_calibration = 0;
    for calculation in &calculation_list {
        // println!("\n{:?}", calculation);
        if try_triple_operation(calculation.result, 0, &calculation.factors) {
            // is good, let's add it
            // println!("is valid");
            total_calibration += calculation.result;
        }
    }

    println!("Total calibration value is {}", total_calibration);
    // 227615740238334
    // yes! my part one solution was good enough that it wasn't difficult to modify for concatenation

    Ok(())
}