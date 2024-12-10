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
    // let's store the size for part 2 so that we don't have to do too much searching
    #[derive(Debug)]
    struct FileProperties {
        file_id: usize,
        length: usize,
        location: usize,
    }
    let mut file_lengths: Vec<FileProperties> = Vec::new();

    for (is_file, entry) in raw_files.iter().enumerate() {
        if is_file % 2 == 0 {
            // file
            file_lengths.push(FileProperties{file_id: file_number, length: *entry as usize, location: expanded_files.len()});
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
    let mut basic_expanded_files = expanded_files.clone();
    while basic_expanded_files.iter().position(|&x| x == usize::MAX) != None {
        let first_empty = basic_expanded_files.iter().position(|&x| x == usize::MAX).unwrap();
        let move_char = basic_expanded_files.pop().unwrap();
        basic_expanded_files[first_empty] = move_char;
        number_of_moves += 1;
    }

    println!("There are {} moves", number_of_moves);
    // println!("{:?}", expanded_files);

    let mut checksum = 0;
    for (i, entry) in basic_expanded_files.iter().enumerate() {
        if *entry == usize::MAX {
            break
        }
        else {
            checksum += i * entry;
        }
    }

    println!("The checksum is {}", checksum);
    // 6301895872542

    // fn contains_subslice<T: PartialEq>(data: &[T], needle: &[T]) -> bool {
    //     data
    //         .windows(needle.len())
    //         .any(|w| w == needle)
    // }

    fn position_subslice<T: PartialEq>(data: &[T], needle: &[T]) -> Option<usize> {
        data
            .windows(needle.len())
            .enumerate()
            .find(|&(_, w)| w == needle)
            .map(|(i, _)| i)
    }

    let mut complex_expanded_files = expanded_files.clone();


    while !file_lengths.is_empty() {
        let moving_block = file_lengths.pop().unwrap();
        // println!("\nmoving block: {:?}", moving_block);
        let brute_force_gap: Vec<usize> = (0..moving_block.length).map(|_x| usize::MAX).collect::<Vec<usize>>();
        let check_gap = position_subslice(&complex_expanded_files, &brute_force_gap);
        if check_gap != None {
            // we now swap our brute force file/gap
            let gap_pos = check_gap.unwrap();
            if gap_pos < moving_block.location {
                // more to the left, let's move it!
                for i in 0..moving_block.length {
                    complex_expanded_files[gap_pos+i]  = moving_block.file_id;
                    complex_expanded_files[ moving_block.location+i] = usize::MAX;
                }
            }

        }
    }

    let mut checksum = 0;
    for (i, entry) in complex_expanded_files.iter().enumerate() {
        if *entry != usize::MAX {
            checksum += i * entry;
        }
    }

    println!("The checksum is {}", checksum);
    // 6323761685944


    Ok(())
}