use std::fs;

fn day_3_part1(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        result += max_bank(line);
    }
    result
}

/* Example: Take the end of a string like ... 1 9 (rightmost is 9):
Start: best_right = 0, best_num = 0
See 9: d=9, candidate 90, best_num=90, then best_right=9
See 1: d=1, candidate 10*1+9=19, best_num stays 90, best_right stays 9
*/

fn max_bank(line: &str) -> u32 {
    let bytes = line.trim().as_bytes();
    if bytes.len() < 2 {
        // Line must have at least 2 digits to work, by the input of the puzzle
        return 0;
    }

    let mut best_right: u32 = 0;
    let mut best_num: u32 = 0;
    let mut have_right = false;

    // We need to iterate from right to left
    for &b in bytes.iter().rev() {
        let digit = (b - b'0') as u32; // Trick to convert ASCII char to int, but works only in UTF-8 encodings
        // digit now is a number in the range 0-9.
        if have_right {
            best_num = best_num.max(10 * digit + best_right); // best_num keeps track of the best number returned at the end
            // Since it must be of 2 digits, it multiplies by ten the leftmost one and then adds up the best right number found.
            // This happens only if we have a number to the right, so not at the first attempt in the line
        }
        best_right = best_right.max(digit); // Here it replaces the best second digit (the right one) whenever a bigger one
        // is found.
        have_right = true;
    }
    best_num
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
    println!("Result is: \n{}", day_3_part1(&input))
}
