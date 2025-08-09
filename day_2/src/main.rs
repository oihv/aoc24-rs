use std::fs;

#[derive(PartialEq)]
enum ReportTrend {
    Increasing,
    Decreasing,
    Flat
}

fn get_report_trend(prev: i32, curr: i32) -> ReportTrend {
    if prev > curr {
        ReportTrend::Decreasing
    } else if curr > prev {
        ReportTrend::Increasing
    } else {
        ReportTrend::Flat
    }
}

fn get_report_disparity(prev: i32, curr:i32) -> u32 {
    prev.abs_diff(curr)
}

fn check_report_safety(report: String) -> bool {
    let mut numbers = report.split(" ");

    let mut prev = numbers.next().unwrap().parse().unwrap();

    let curr = numbers.next().unwrap().parse().unwrap();

    let trend = get_report_trend(prev, curr);

    if trend == ReportTrend::Flat {
        return false;
    }

    let disparity = get_report_disparity(prev, curr);
    if disparity > 3 || disparity < 1 {
        return false;
    }
    
    prev = curr;

    while let Some(curr) = numbers.next() {
        let curr = curr.parse().unwrap();
        if get_report_trend(prev, curr) != trend {
            return false;
        }  

        let disparity = get_report_disparity(prev, curr);
        if disparity > 3 || disparity < 1 {
            return false;
        }

        prev = curr;
    } 
    true
}

fn get_safe_report_sum(data: String) -> u32 {
    let mut reports = data.lines();

    let mut sum = 0;

    while let Some(line) = reports.next() {
        if check_report_safety(line.to_string()) {
            sum += 1;
        }
    }
    sum
}

fn main() {
    println!("Testing\nPart 1: {}", get_safe_report_sum(fs::read_to_string("src/test.txt").expect("Cant't read the test file")));

    println!("Real Input\nPart 1: {}", get_safe_report_sum(fs::read_to_string("src/input.txt").expect("Cant't read the input file")));
}
