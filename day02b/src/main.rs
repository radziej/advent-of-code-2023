use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut set_powers: Vec<u32> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");

        let (_, draws) = line.split_once(": ").expect("Line must start with game ID");
        // let (_, id) = game_id.split_once(" ").expect("Game ID must have format 'Game X'");

        let mut minimum_set: [u32; 3] = [0, 0, 0];
        for draw in parse_game(draws) {
            for i in 0..draw.len() {
                if draw[i] > minimum_set[i] {
                    minimum_set[i] = draw[i];
                }
            }
        }

        set_powers.push(minimum_set.into_iter().product::<u32>())
    }
    println!("Sum of all minimum power sets: {}", set_powers.iter().sum::<u32>());
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_game(draws: &str) -> Vec<[u32; 3]> {
    let mut game_result: Vec<[u32; 3]> = Vec::new();
    for draw in draws.split("; ") {
        let mut draw_result: [u32; 3] = [0, 0, 0];
        for color_amount in draw.split(", ") {
            let (amount, color) = color_amount.split_once(" ").unwrap();
            let amount = amount.parse::<u32>().unwrap();
            match color {
                "red" => draw_result[0] = amount,
                "green" => draw_result[1] = amount,
                "blue" => draw_result[2] = amount,
                _ => panic!("Unexpected color"),
            }
        }
        game_result.push(draw_result);
    }
    return game_result;
}
