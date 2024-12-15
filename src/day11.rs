use aoc_2024::*;

pub fn day11(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 11");

    let data = read_input(11, input_type, manual_name)?;
    
    let mut initial_stones = Vec::new();
    
    for col in &data[0] {
        initial_stones.push(str_to_u64(&col));
    }
    
    println!("Initial stones: {:?}", initial_stones);
    let mut current_stones = initial_stones.clone();
    
    for _blink in 0..25 {
        
        let mut new_stones = Vec::new();
        
        // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. 
        // The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the 
        // new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 
        // is engraved on the new stone.
        for old_stone in current_stones {
            
            if old_stone == 0 {
                new_stones.push(1);
            }
            else if old_stone.to_string().len() % 2 == 0 {
                let old_numer_str = old_stone.to_string();
                let old_length = old_numer_str.len();
                let part1 = str_to_u64(&old_numer_str[0..old_length/2]);
                let part2 = str_to_u64(&old_numer_str[old_length/2..old_length]);
                new_stones.push(part1);
                new_stones.push(part2);
            }
            else {
                new_stones.push(old_stone * 2024);
            }
        }
        
        current_stones = new_stones;
        // println!("{}: {:?}", _blink, current_stones);
        // println!("{}: {:?}", _blink, current_stones.len());
        
    }
    
    println!("Thee are now {} stones", current_stones.len());
    // 408070 too high
    // 231278 yay!


    // Part 2 doesn't work brute force
    // can we go recursive?

    fn count_stone_evolutions(current_reverse_cycle: usize, current_stone: u64) -> u64 {

        let mut return_value = 0;
        if current_reverse_cycle == 0 {
            // print!("{} ", current_stone);
            // last cycle, we return the number of the stone evolutions, not the number itself!
            if current_stone.to_string().len() % 2 == 0{
                return_value = 2;
            }
            else {
                return_value = 1;
            }
        }
        else {
            // process the stone, and return a sum?
            if current_stone == 0 {
                return_value = count_stone_evolutions(current_reverse_cycle-1, 1);
            }
            else if current_stone.to_string().len() % 2 == 0 {
                let old_numer_str = current_stone.to_string();
                let old_length = old_numer_str.len();
                let part1 = str_to_u64(&old_numer_str[0..old_length/2]);
                let part2 = str_to_u64(&old_numer_str[old_length/2..old_length]);
                return_value = count_stone_evolutions(current_reverse_cycle-1, part1) +
                    count_stone_evolutions(current_reverse_cycle-1, part2);
            }
            else {
                return_value = count_stone_evolutions(current_reverse_cycle-1, current_stone * 2024);
            }

        }

        return_value
    }

    let mut recursive_count = 0;
    // for blink in 0..6 {
        recursive_count = 0;
        // println!("-------");
        for stone in &initial_stones {
            recursive_count += count_stone_evolutions(74, *stone);
        }
        // println!();
        println!("Recursive count for {} is {}", 75, recursive_count);
    // }



    Ok(())
}