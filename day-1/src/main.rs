use std::{fs, panic};

fn day_1(input: &str) -> i32 {
    let total_pos = 100;
    let mut position = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue; //Skip empty lines
        }

        let (direction_str, amount_str) = line.split_at(1);
        let amount_result: Result<i32, std::num::ParseIntError> = amount_str.parse();

        let amount = match amount_result {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Error parsing amount '{}': {}", amount_str, e);
                panic!("Could not parse input.");
            }
        };

        let old_position = position;
        let intermediate_position = match direction_str {
            "R" => old_position + amount,
            "L" => old_position - amount,
            _ => continue,
        };

        let mut count_this_move = 0;

        if old_position < intermediate_position {
            // Moving right
            // Count multiples of total_pos in (old_position, intermediate_position]
            let start_multiple_index = old_position.div_euclid(total_pos);
            let end_multiple_index = intermediate_position.div_euclid(total_pos);

            count_this_move = end_multiple_index - start_multiple_index;
            if intermediate_position % total_pos == 0 && old_position % total_pos == 0 {
                // If both start and end are multiples (e.g. 0 to 100)
                // old=0, inter=100. start_idx=0, end_idx=1. count = 1.
                // old=100, inter=200. start_idx=1, end_idx=2. count = 1.
            }
        } else if old_position > intermediate_position {
            // Moving left
            // Count multiples of total_pos in (intermediate_position, old_position)
            let start_multiple_index = intermediate_position.div_euclid(total_pos);
            let end_multiple_index = old_position.div_euclid(total_pos);

            count_this_move = end_multiple_index - start_multiple_index;
            if old_position % total_pos == 0 && intermediate_position % total_pos != 0 {
                // If we started at a multiple and moved left (e.g., 0 L5 -> -5)
                // start_idx=-1, end_idx=0. count = 1.
                // We need to uncount the starting 0 in this specific case.
                count_this_move -= 1;
            } else if intermediate_position % total_pos == 0 && old_position % total_pos != 0 {
                // If we land on 0 and didn't start at 0. (e.g. 50 L50 -> 0)
                // start_idx=0, end_idx=0. count=0. Add 1.
                count_this_move += 1;
            }
        }

        zero_count += count_this_move;
        position = intermediate_position.rem_euclid(total_pos);
    }

    zero_count
}

fn main() {
    let file_path = "input.txt";
    let input = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file `{}`: {}", file_path, error);
            panic!("Could not read input file.");
        }
    };
    println!("Result is: \n{}", day_1(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_provided_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(day_1(input), 6);
    }

    #[test]
    fn test_large_right_rotation() {
        // Initial position 50
        // R1000 -> 1050. Counts 10 full cycles (100, 200, ..., 1000).
        let input = "R1000";
        assert_eq!(day_1(input), 10);
    }
}
