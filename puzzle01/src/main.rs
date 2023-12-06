use std::{env, fs::File, path::Path, io::{self, BufRead, Error, Seek, SeekFrom}};

fn main() -> io::Result<()>{
    // Check if text file was given
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Syntax: puzzle01 [input file]");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, ""))
    }

    // Open given text file
    let file_path = &args[1];
    let file = File::open(Path::new(file_path)).unwrap();
    let mut reader = io::BufReader::new(file);    

    println!("Puzzle 01 part 1 result: {}", puzzle01_part1(&mut reader)?);
    reader.seek(SeekFrom::Start(0))?; // Reset reader to file start
    println!("Puzzle 01 part 2 result: {}", puzzle01_part2(&mut reader)?);

    Ok(())
}

fn puzzle01_part1(reader: &mut io::BufReader<File>) -> Result<u64, Error> {
    let mut sum:u64 = 0;

    for line in reader.lines() {
        let line = line?;

        // Find the first and last digit in the line
        if let Some((first_digit, last_digit)) = find_first_and_last_digit(&line) {
            // use for debugging
            //println!("First digit: {}, Last digit: {}", first_digit, last_digit);
            sum += first_digit * 10 + last_digit;
        }        
    }

    Ok(sum)
}

fn find_first_and_last_digit(s: &str) -> Option<(u64, u64)> {
    // Find the first digit
    let first_digit = char_to_u64(s.chars().find(|&c| c.is_digit(10)));

    // Find the last digit
    let last_digit = char_to_u64(s.chars().rev().find(|&c| c.is_digit(10)));

    // If both are found, return them
    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None,
    }
}

fn char_to_u64(c: Option<char>) -> Option<u64> {
    c.and_then(|char| char.to_digit(10).map(|d| d as u64))
}

fn puzzle01_part2(reader: &mut io::BufReader<File>) -> Result<u64, Error> {
    let mut sum:u64 = 0;

    for line in reader.lines() {
        let line = line?;

        // Find the first and last digit in the line
        if let Some((first_digit, last_digit)) = find_first_and_last_digit_including_spelled_out(&line) {
            println!("Line: {}", line);
            println!("First digit: {}, Last digit: {}", first_digit, last_digit);
            sum += first_digit * 10 + last_digit;
        }        
    }

    Ok(sum)
}

fn find_first_and_last_digit_including_spelled_out(s: &str) -> Option<(u64, u64)> {
    let digit_strings = [("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    let first_digit = find_digit_in_str(s, &digit_strings, true);
    let last_digit = find_digit_in_str(s, &digit_strings, false);

    // If both are found, return them
    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None,
    }
}

fn find_digit_in_str(s: &str, digit_strings: &[(&str, u64)], is_first: bool) -> Option<u64> {
    let mut current_pos = 0;
    let mut last_found_digit: Option<u64> = None;

    while current_pos < s.len() {
        let remaining = &s[current_pos..];

        if let Some((digit_value, len)) = find_digit_or_string(remaining, digit_strings) {
            if is_first {
                return Some(digit_value);
            } else {
                last_found_digit = Some(digit_value);
                current_pos += len;
            }
        } else {
            current_pos += 1;
        }
    }

    last_found_digit
}

fn find_digit_or_string(s: &str, digit_strings: &[(&str, u64)]) -> Option<(u64, usize)> {
    if s.is_empty() {
        return None;
    }

    if let Some(c) = s.chars().next() {
        if c.is_digit(10) {
            return Some((c.to_digit(10).unwrap() as u64, 1));
        }
    }

    for &(word, value) in digit_strings {
        if s.starts_with(word) {
            return Some((value, word.len()));
        }
    }

    None
}