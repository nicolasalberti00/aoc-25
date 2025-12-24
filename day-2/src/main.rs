fn day_2_part_1(input: &str) -> Result<u64, String> {
    if input.trim().is_empty() {
        return Ok(0);
    }

    let mut sum: u64 = 0;

    for r in input.split(",").map(str::trim) {
        let (start, end) = parse_range(r)?;

        for n in start..=end {
            if get_double_digit(n).is_some() {
                sum += n;
            }
        }
    }

    Ok(sum)
}

fn day_2_part_2(input: &str) -> Result<u64, String> {
    if input.trim().is_empty() {
        return Ok(0);
    }

    let mut sum: u64 = 0;

    for r in input.split(",").map(str::trim) {
        let (start, end) = parse_range(r)?;

        for n in start..=end {
            if get_repeated_string(n) {
                sum += n;
            }
        }
    }

    Ok(sum)
}

fn get_double_digit(n: u64) -> Option<u64> {
    let s = n.to_string();
    if s.len() % 2 != 0 || s.is_empty() {
        return None;
    }

    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);

    if left == right {
        left.parse::<u64>().ok()
    } else {
        None
    }
}

fn get_repeated_string(n: u64) -> bool {
    let s = n.to_string();
    let length = s.len();
    if length < 2 {
        return false;
    }

    // If total length is not multiple of block_len, there could not be any
    // substring, so we return false directly because the string could not
    // be composed without having an incomplete pattern check.
    // Example: "121" does not work, "1212" works
    for block_len in 1..=length / 2 {
        if length % block_len != 0 {
            continue; // go to the next block_len
        }

        // Count how many times we repeat block_len to arrive to length chars
        // Example: len=12, block_len=3 -> times=4 ("abc" * 4 = 12)
        let times = length / block_len;
        let block = &s[..block_len]; // Takes a candidate pattern to analyze
        if block.repeat(times) == s {
            return true;
            // If the block repeated is equal to the start string then we have found
            // a repeatable substring
        }
    }

    false
}

fn parse_range(range: &str) -> Result<(u64, u64), String> {
    let (a, b) = range
        .split_once("-")
        .ok_or_else(|| format!("Invalid range: {range}"))?; //See notes

    let start: u64 = a
        .parse()
        .map_err(|_| format!("Start is not a number: {a}"))?;

    let end: u64 = b.parse().map_err(|_| format!("End is not a number: {b}"))?;

    if start > end {
        return Err(format!("Range inverted: {start}-{end}"));
    }
    Ok((start, end))
}

fn main() {
    let file_path = "input.txt";
    let input = std::fs::read_to_string(file_path).expect("Cannot read input.txt");

    match day_2_part_1(&input) {
        Ok(sum) => println!("Result of part_1 is {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }

    match day_2_part_2(&input) {
        Ok(sum) => println!("Result of part_2 is {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_number() {
        assert_eq!(get_double_digit(11), Some(1));
        assert_eq!(get_double_digit(212212), Some(212))
    }

    #[test]
    fn should_return_false() {
        assert_eq!(get_double_digit(123456), None);
        assert_eq!(get_double_digit(121), None)
    }

    #[test]
    fn parse_range_ok() {
        assert_eq!(parse_range("212212-212218"), Ok((212212, 212218)));
    }

    #[test]
    fn parse_range_missing_dash_is_err() {
        assert!(parse_range("212212212218").is_err());
    }

    #[test]
    fn parse_range_non_numeric_is_err() {
        assert!(parse_range("abc-10").is_err());
        assert!(parse_range("10-xyz").is_err());
    }

    #[test]
    fn parse_range_inverted_is_err() {
        assert!(parse_range("20-10").is_err());
    }
}
