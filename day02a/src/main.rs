use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut valid_game_ids: Vec<u32> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");

        let (game_id, draws) = line.split_once(": ").expect("Line must start with game ID");
        let (_, id) = game_id.split_once(" ").expect("Game ID must have format 'Game X'");

        if verify_game(draws) {
            valid_game_ids.push(id.parse::<u32>().expect("Game ID must be numerical"));
        }
    }
    println!("Sum of all valid game IDs: {}", valid_game_ids.iter().sum::<u32>());
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn verify_game(draws: &str) -> bool {
    for draw in draws.split("; ") {
        for color_amount in draw.split(", ") {
            let (amount, color) = color_amount.split_once(" ").unwrap();
            let amount = amount.parse::<u32>().unwrap();
            if match color {
                "red" => amount > 12,
                "green" => amount > 13,
                "blue" => amount > 14,
                _ => panic!("Unexpected color"),
            } {
                return false;
            }
        }
    }
    return true;
}