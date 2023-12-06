use std::env;

fn main() {
    // Open given text file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Syntax: puzzle01 [input file]");
        return
    }

    println!("Puzzle 01 part 1 result: {}", puzzle01_part1());
    println!("Puzzle 01 part 2 result: {}", puzzle01_part2());
}

fn puzzle01_part1() -> u64 {
    0
}

fn puzzle01_part2() -> u64 {
    0
}