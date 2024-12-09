use aoc_2024::*;

pub fn day09(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 09");


    let data = read_input(09, input_type, manual_name)?;

    let raw_files : Vec<u32>= str_to_chars(&data[0][0]).iter().map(|c| c.to_digit(10).unwrap()).collect();
    // println!("{:?}", raw_files);

    let mut expanded_files = Vec::new();
    // println!("{:?}", raw_files)

    // is this too much brute force?
    let mut file_number: usize = 0;
    for (is_file, entry) in raw_files.iter().enumerate() {
        if is_file % 2 == 0 {
            // file
            for _i in 0..*entry {
                expanded_files.push(file_number);
            }
            file_number += 1;
        }
        else {
            for _i in 0..*entry {
                expanded_files.push(usize::MAX);
            }
        }
    }
    // println!("{:?}", expanded_files);

    let mut number_of_moves = 0;
    //assert_eq!(a.iter().position(|&x| x == 5), None);
    while expanded_files.iter().position(|&x| x == usize::MAX) != None {
        let first_empty = expanded_files.iter().position(|&x| x == usize::MAX).unwrap();
        let move_char = expanded_files.pop().unwrap();
        expanded_files[first_empty] = move_char;
        number_of_moves += 1;
    }

    println!("There are {} moves", number_of_moves);
    // println!("{:?}", expanded_files);

    let mut checksum = 0;
    for (i, entry) in expanded_files.iter().enumerate() {
        if *entry == usize::MAX {
            break
        }
        else {
            checksum += i * entry;
        }
    }

    println!("The checksum is {}", checksum);
    // 6301895872542


    Ok(())
}