use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let file_path = "./input.txt";

    let mut cards: HashMap<u32, u32> = HashMap::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let (header, content) = line.split_once(": ").unwrap();
        let card_id = header.split_whitespace().last().unwrap().parse::<u32>().unwrap();
        cards.entry(card_id).and_modify(|v| *v += 1).or_insert(1);

        let (winning_numbers, drawn_numbers) = content.split_once(" | ").unwrap();
        let winners: HashSet<u32> = winning_numbers.split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect();
        let draws: HashSet<u32> = drawn_numbers.split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect();

        let number_of_card = cards.get(&card_id).unwrap().clone();
        let matches = winners.intersection(&draws).count() as u32;
        for offset in 1..=matches {
            cards.entry(card_id + offset).and_modify(|v| *v += number_of_card).or_insert(number_of_card);
        }
    }

    println!("Total number of scratch cards: {}", cards.values().sum::<u32>());

}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}