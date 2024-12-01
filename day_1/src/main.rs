use std::env;
use std::fs;
use std::collections::HashMap;


fn parse_columns(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let num1 = split.next().unwrap().parse::<i32>().unwrap();
        let num2 = split.next().unwrap().parse::<i32>().unwrap();
        column1.push(num1);
        column2.push(num2);
    }
    (column1, column2)
}

fn part1(contents: &str) {
    println!("Part 1");
    let (mut column1, mut column2) = parse_columns(&contents);
    column1.sort();
    column2.sort();

    let mut absolute_differences: Vec<i32> = Vec::new();
    for i in 0..column1.len() {
        let diff = &column1[i] - &column2[i];
        absolute_differences.push(diff.abs());
    }
    let sum: i32 = absolute_differences.iter().sum();
    println!("Sum of absolute differences: {}", sum);
}

fn count_numbers_in_column(column: &Vec<i32>) -> HashMap<i32, i32> {
    let mut frequency: HashMap<i32, i32> = HashMap::new();
    for num in column {
        let count = frequency.entry(*num).or_insert(0);
        *count += 1;
    }
    frequency
}

fn part2(contents: &str) {
    println!("Part 2");
    let (column1, column2) = parse_columns(&contents);
    
    let frequency2 = count_numbers_in_column(&column2);
    println!("Frequencies of column 2: {:?}", frequency2);

    let mut sum_of_multiples = 0;
    for num in &column1 {
        sum_of_multiples += num * frequency2.get(&num).copied().unwrap_or(0);
    }
    println!("Sum of multiples: {}", sum_of_multiples);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];
    let file_path = &args[2];

    println!("Reading file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    println!("With text:\n{contents}");

    if part == "1" {
        part1(&contents);
    } else if part == "2" {
        part2(&contents);
    } else {
        println!("Invalid part number");
    }
}
