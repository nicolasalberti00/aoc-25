use std::{fs, panic};

fn day_1(input: &str) {
    let (direction_str, amount_str) = input.split_at(1);
    let amount_result: Result<i32, std::num::ParseIntError> = amount_str.parse();

    let amount = match amount_result {
        Ok(num) => {
            println!("Successfully parsed amount: {}", num);
            num
        }
        Err(e) => {
            eprintln!("Error parsing amount '{}': {}", amount_str, e);
            panic!("Could not parse input.");
        }
    };
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
