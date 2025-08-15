use std::fs;

fn count_xmas(line: &String) -> u32 {
    let mut line = line.clone();
    let mut line_clone = line.clone();

    let mut count = 0;

    while let Some(index) = line.rfind("XMAS") {
        count += 1;
        dbg!(&line);
        line.truncate(index);
    }

    while let Some(index) = line_clone.rfind("SAMX") {
        count += 1;
        dbg!(&line_clone);
        line_clone.truncate(index);
    }

    count
}

fn process(path: &str) -> u32 {
    let input = fs::read_to_string(path).expect("File not found");

    let mut sum = 0;
    // Check in each line
    for line in input.lines() {
        sum += count_xmas(&(line).to_string());
    }
    // Convert vector for easy access
    let mut input_vec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut line_vec: Vec<char> = Vec::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        input_vec.push(line_vec);
    }

    // Get the dimension of the input
    let width = input_vec[0].len();
    let height = input_vec.len();

    // Check vertical
    for i in 0..width {
        let mut vert_line = String::new();
        for j in 0..height {
            vert_line.push(input_vec[j][i]);
        }
        sum += count_xmas(&vert_line);
    }
    // Check diagonal (top left - bottom right)
    for i in 1..height-4 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < height && inc < width {
            diagonal_line.push(input_vec[i+inc][inc]);
            inc += 1;
        }
        sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    for i in 0..width-4 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < width && inc < height {
            diagonal_line.push(input_vec[inc][i+inc]);
            inc += 1;
        }
        sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    // Check diagonal (top right - bottomm left)
    for i in 5..height {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < height && inc < width {
            diagonal_line.push(input_vec[i+inc][inc]);
            inc += 1;
        }
        // sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    for i in 1..width-4 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < width && inc < height {
            diagonal_line.push(input_vec[inc][i+inc]);
            inc += 1;
        }
        // sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    sum
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_input() {
        assert_eq!(process("src/test.txt"), 18);
    }
}

