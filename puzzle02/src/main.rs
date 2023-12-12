use std::{env, fs::File, path::Path, io::{self, Error, Seek, SeekFrom, BufRead}, collections::HashMap};

// TODO: Result for input.txt is "3472" at the moment which is too high

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

    println!("Puzzle 02 part 1 result: {}", puzzle02_part1(&mut reader)?);
    reader.seek(SeekFrom::Start(0))?; // Reset reader to file start
    println!("Puzzle 02 part 2 result: {}", puzzle02_part2(&mut reader)?);

    Ok(())
}

fn puzzle02_part1(reader: &mut io::BufReader<File>) -> Result<u64, Error> {
    let mut sum = 0u64;

    for line in reader.lines() {
        let line = line?;

        match parse_game_data(&line) {
            Ok((game_id, color_counts)) => {
                let mut too_many = false;
                for map_entry in color_counts {
                    let red_count = map_entry.get("red").unwrap_or(&0);
                    let green_count = map_entry.get("red").unwrap_or(&0);
                    let blue_count = map_entry.get("red").unwrap_or(&0);
                   
                    if (red_count > &12) || (green_count > &13) || (blue_count > &14) {
                        too_many = true;
                        break;
                    }
                }

                if !too_many {
                    sum += game_id;
                }
            },
            Err(e) => {
                println!("Parsing error: {e}");
                println!("Line: {line}");
            }
        }
    }
    
    Ok(sum)
}

fn parse_game_data(input: &str) -> Result<(u64, Vec<HashMap<String, u32>>), String> {
    let parts: Vec<&str> = input.split(": ").collect();
    if parts.len() != 2 {
        return Err("Invalid input format".to_string());
    }

    let game_id: u64 = parts[0].replace("Game ", "").parse::<u64>().map_err(|_| "Invalid game ID".to_string())?;

    let mut color_counts_list = Vec::new();
    for group in parts[1].split("; ") {
        let mut color_counts = HashMap::new();
        for count in group.split(", ") {
            let count_parts: Vec<&str> = count.split(' ').collect();
            if count_parts.len() != 2 {
                return Err("Invalid color count format".to_string());
            }

            let color = count_parts[1].to_lowercase();
            let number = count_parts[0].parse::<u32>().map_err(|_| "Invalid number format".to_string())?;
            color_counts.insert(color, number);
        }
        color_counts_list.push(color_counts);
    }

    Ok((game_id, color_counts_list))
}

fn puzzle02_part2(reader: &mut io::BufReader<File>) -> Result<u64, Error> {
    Ok(0)
}