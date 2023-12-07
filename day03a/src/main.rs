use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let file_path = "./input.txt";

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in read_lines(file_path).expect("Should be able to read input file") {
        let line = line.expect("Should be able to read line of input file");
        grid.push(line.chars().collect());
    }
    let ymax = grid.len();
    let xmax = grid.first().unwrap().len();


    let mut part_numbers: HashMap<[usize; 2], usize> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            // Skip everything that is not a symbol
            match item {
                '.' => continue,
                '0'..='9' => continue,
                _ => {}
            }

            // Look for adjacent numbers
            for neighbor in neighbors([x, y], 0, xmax, 0, ymax) {
                match grid[neighbor[1]][neighbor[0]] {
                    '0'..='9' => {
                        let root = find_root(&grid[neighbor[1]], neighbor[0]);
                        if !part_numbers.contains_key(&[root, neighbor[1]]) {
                            part_numbers.insert([root, neighbor[1]], parse_number(&grid[neighbor[1]], root));
                        }
                    }
                    _ => {}
                };
            }
        }
    }

    println!("Sum of all part numbers: {}", part_numbers.values().sum::<usize>());
}

fn read_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn neighbors(position: [usize; 2], xmin: usize, xmax: usize, ymin: usize, ymax: usize) -> Vec<[usize; 2]> {
    let mut x_options = vec![position[0]];
    if position[0] > xmin {
        x_options.push(position[0] - 1);
    }
    if position[0] + 1 < xmax {
        x_options.push(position[0] + 1);
    }

    let mut y_options = vec![position[1]];
    if position[1] > ymin {
        y_options.push(position[1] - 1);
    }
    if position[1] + 1 < ymax {
        y_options.push(position[1] + 1);
    }

    let mut options: Vec<[usize; 2]> = Vec::new();
    for x in &x_options {
        for y in &y_options {
            if x == &position[0] && y == &position[1] {
                continue;
            }

            options.push([*x, *y]);
        }
    }
    return options;
}

fn find_root(row: &Vec<char>, pos: usize) -> usize {
    let mut start = pos;
    for position in (0..pos).rev() {
        match row[position] {
            '0'..='9' => start = position,
            _ => break,
        }
    }
    return start;
}

fn parse_number(row: &Vec<char>, start: usize) -> usize {
    let mut end = start + 1;
    while end < row.len() {
        match row[end] {
            '0'..='9' => end += 1,
            _ => break,
        }
    }

    return (&row[start..end]).iter().collect::<String>().parse::<usize>().expect("Parts need to be numbers");
}