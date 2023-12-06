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
            println!("First digit: {}, Last digit: {}", first_digit, last_digit);
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
        if let Some((first_digit, last_digit)) = find_first_and_last_digit(&line) {
            println!("First digit: {}, Last digit: {}", first_digit, last_digit);
            sum += first_digit * 10 + last_digit;
        }        
    }

    Ok(sum)
}