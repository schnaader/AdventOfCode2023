use std::{env, fs::File, path::Path, io::{self, Error, Seek, SeekFrom, BufRead}, collections::HashMap, cmp::max};

fn main() -> io::Result<()>{
    // Check if text file was given
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Syntax: puzzle02 [input file]");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, ""))
    }

    // Open given text file
    let file_path = &args[1];
    let file = File::open(Path::new(file_path)).unwrap();
    let mut reader = io::BufReader::new(file);

    // Read file content into string array
    let mut map: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        map.push(line);
    }    

    println!("Puzzle 02 part 1 result: {}", puzzle02_part1(&map)?);
    println!("Puzzle 02 part 2 result: {}", puzzle02_part2(&map)?);

    Ok(())
}

fn puzzle02_part1(map: &Vec<String>) -> Result<u64, Error> {
    let mut sum = 0u64;
    let w = map[0].len();
    let h = map.len();

    for (x, row) in map.iter().enumerate() {
        for (y, &cell) in row.as_bytes().iter().enumerate() {
            // symbol?
            if !cell.is_ascii_digit() && cell != b'.' {
                // scan neighbor cells
                for xn in x.saturating_sub(1)..=(x + 1).min(h - 1) {
                    for yn in y.saturating_sub(1)..=(y + 1).min(w - 1) {
                        if map[x].as_bytes()[y].is_ascii_digit() {
                            // TODO: Scan for numbers
                        }
                    }
                }                
            }
        }
    }

    Ok(sum)
}

fn puzzle02_part2(map: &Vec<String>) -> Result<u64, Error> {
    let mut sum = 0u64;
   
    Ok(sum)
}