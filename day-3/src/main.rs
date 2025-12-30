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

fn day_3_part_2(input: &str) -> u64 {
    input
        .lines() // For all lines
        .filter(|l| !l.trim().is_empty()) // Keep the not empty lines
        .map(|l| max_bank_with_batteries(l, 12)) // Transform each line into the number computed by the function
        .sum() // Sum them all up and return the result
}

fn max_bank_with_batteries(line: &str, batteries: usize) -> u64 {
    let bytes = line.trim().as_bytes();
    let digits = bytes.len();
    if digits < batteries {
        // Line must have at least 12 digits to work, by the input of the second part of the puzzle
        return 0;
    }

    let mut digits_allowed_to_drop = bytes.len() - batteries;
    let mut stack: Vec<u8> = Vec::with_capacity(digits);

    for &b in bytes {
        let digit = b - b'0'; // Same trick as part 1

        // We can substitute the value from the subsequence whenever we have:
        // 1. Available digits to drop: since we still have space to find a better value, in our case
        // it's the length of the line minus the number of batteries needed, for the puzzle 12.
        // 2. The stack is not empty
        // 3. If the last digit in the stack (that is still built as a subsequence) is smaller than the
        // one we just found, because in the order it is required to have the greates possible subsequence
        while digits_allowed_to_drop > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
            stack.pop();
            digits_allowed_to_drop -= 1;
        }
        stack.push(digit);
    }

    // If we still have removals left, drop from the end (least significant side)
    while digits_allowed_to_drop > 0 {
        stack.pop();
        digits_allowed_to_drop -= 1;
    }

    // We build the result value from the stack with an accumulator (val)
    /* Example: If digits are [4, 3, 4]
     * start val=0
     * after 4: 0*10+4 = 4
     * after 3: 4*10+3 = 43
     * after 4: 43*10+4 = 434
     */
    let mut val: u64 = 0;
    for &d in stack.iter().take(batteries) {
        val = val * 10 + d as u64;
    }
    val
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
    println!("Result of part 1 is: \n{}", day_3_part1(&input));
    println!("Result of part 2 is: \n{}", day_3_part_2(&input))
}
