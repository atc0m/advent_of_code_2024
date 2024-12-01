use std::env;
use std::fs;


fn parse_input_part1(contents: &str) -> (Vec<i32>, Vec<i32>) {
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
    let (mut column1, mut column2) = parse_input_part1(&contents);
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

fn part2(contents: &str) {
    println!("Part 2");
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
