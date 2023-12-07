use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    let mut calibration_values: Vec<u32> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let first_digit = line.chars().find(|&c| c.is_ascii_digit()).expect("Line should contain a digit");
        let last_digit = line.chars().rfind(|&c| c.is_ascii_digit()).expect("Line should contain a digit");

        calibration_values.push(format!("{}{}", first_digit, last_digit).parse::<u32>().expect("Digits should combine to small number"));
    }
    println!("Sum of all calibration values: {}", calibration_values.iter().sum::<u32>());
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
