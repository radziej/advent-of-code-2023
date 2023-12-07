use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "./input.txt";

    // assert_eq!(find_digit(&"asdasdasdasd"), None);
    // assert_eq!(rfind_digit(&"asdasdasdasd"), None);
    // assert_eq!(find_digit(&"asdasdonetwoasdasd"), Some(1));
    // assert_eq!(rfind_digit(&"asdasdonetwoasdasd"), Some(2));
    // assert_eq!(find_digit(&"stvgmjgnine2vvsnjhlzkstwo5szsbvzjdzb"), Some(9));
    // assert_eq!(rfind_digit(&"stvgmjgnine2vvsnjhlzkstwo5szsbvzjdzb"), Some(5));

    let mut calibration_values: Vec<u32> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        let first_digit = find_digit(&line).expect("Line should contain a digit");
        let last_digit = rfind_digit(&line).expect("Line should contain a digit");

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

fn find_digit(string: &str) -> Option<u32> {
    for i in 0..string.len() {
        let mut result = match &string[i..i + 1] {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }

        if i + 3 >= string.len() {
            continue;
        }
        result = match &string[i..i + 3] {
            "one" => 1,
            "two" => 2,
            "six" => 6,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }

        if i + 4 > string.len() {
            continue;
        }
        result = match &string[i..i + 4] {
            "four" => 4,
            "five" => 5,
            "nine" => 9,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }


        if i + 5 > string.len() {
            continue;
        }
        result = match &string[i..i + 5] {
            "three" => 3,
            "seven" => 7,
            "eight" => 8,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }
    }

    return None;
}

fn rfind_digit(string: &str) -> Option<u32> {
    for i in (0..string.len()).rev() {
        let mut result = match &string[i..i + 1] {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }

        if i + 3 > string.len() {
            continue;
        }
        result = match &string[i..i + 3] {
            "one" => 1,
            "two" => 2,
            "six" => 6,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }

        if i + 4 > string.len() {
            continue;
        }
        result = match &string[i..i + 4] {
            "four" => 4,
            "five" => 5,
            "nine" => 9,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }

        if i + 5 > string.len() {
            continue;
        }
        result = match &string[i..i + 5] {
            "three" => 3,
            "seven" => 7,
            "eight" => 8,
            _ => 0,
        };
        if result != 0 {
            return Some(result);
        }
    }

    return None;
}
