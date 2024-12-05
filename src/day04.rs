use aoc_2024::*;
use diagonal::diagonal_pos_pos;
use diagonal::diagonal_pos_neg;

pub fn day04(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 04");

    fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!original.is_empty());
        let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

        for original_row in original {
            for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
                transposed_row.push(item);
            }
        }

        transposed
    }


    let data = read_input(04, input_type, manual_name)?;
    let mut char_data = Vec::new();
    for row in &data {
        // println!("{:?}", row);
        char_data.push(str_to_chars(&*row[0]));
    }


    // ok, more brute force?
    let xmas = "XMAS";
    let samx = "SAMX";
    let mut total_xmas = 0;

    // horizontal
    for row in &data {
        // println!("{:?}", row[0]);
        total_xmas += row[0].matches(xmas).count();
        total_xmas += row[0].matches(samx).count();
    }

    // vertical
    let vert_data = transpose(char_data.clone());
    for row in vert_data {
        let str_row : String = row.iter().collect::<String>();
        total_xmas += str_row.matches(xmas).count();
        total_xmas += str_row.matches(samx).count();
    }

    // diagonal
    let diag_pos_data = diagonal_pos_pos(&char_data);
    for row in diag_pos_data {
        // the copied moves it from &char to char, needed for collecting
        let str_row : String = row.iter().copied().collect::<String>();
        total_xmas += str_row.matches(xmas).count();
        total_xmas += str_row.matches(samx).count();
    }

    let diag_neg_data = diagonal_pos_neg(&char_data);
    for row in diag_neg_data {
        // the copied moves it from &char to char, needed for collecting
        let str_row : String = row.iter().copied().collect::<String>();
        total_xmas += str_row.matches(xmas).count();
        total_xmas += str_row.matches(samx).count();
    }

    println!("There are {} xmas", total_xmas);
    // 2642 yay!

    // part 2
    let mas = "MAS";
    let sam = "SAM";
    total_xmas = 0;
    for r_y in 1..char_data.len()-1 {
        for c_x in 1..char_data[0].len()-1 {
            // check X as the centre spot
            // limits are ok for that
            if char_data[r_y][c_x] == 'A' {
                // build diagonals (it doesn't matter pos/neg, they are just names)
                let pos_diag = Vec::from([char_data[r_y-1][c_x-1], char_data[r_y][c_x], char_data[r_y+1][c_x+1]]);
                let neg_diag = Vec::from([char_data[r_y+1][c_x-1], char_data[r_y][c_x], char_data[r_y-1][c_x+1]]);
                let pos_diag_str : String = pos_diag.iter().collect::<String>();
                let neg_diag_str : String = neg_diag.iter().collect::<String>();
                if (pos_diag_str.contains(mas) | pos_diag_str.contains(sam)) &
                    (neg_diag_str.contains(mas) | neg_diag_str.contains(sam)) {
                    total_xmas += 1;
                }
            }
        }
    }
    println!("There are now {} X-MAS", total_xmas);

    Ok(())
}