use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let mut numbers = line.split("   ");

        left.push(numbers.next().unwrap().parse().unwrap());
        right.push(numbers.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs()
    }

    println!("{sum}");
}
