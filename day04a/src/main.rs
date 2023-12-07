use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let file_path = "./input.txt";

    let mut total_points = 0;
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (_, content) = line.split_once(": ").unwrap();

        let (winning_numbers, drawn_numbers) = content.split_once(" | ").unwrap();
        let winners: HashSet<u32> = winning_numbers.split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect();
        let draws: HashSet<u32> = drawn_numbers.split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect();

        let matches = winners.intersection(&draws).count() as u32;
        if matches > 0 {
            total_points += 2_u32.pow(matches - 1);
        }
    }

    println!("Total points of scratch cards: {}", total_points);

}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}