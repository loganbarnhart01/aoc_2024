use std::fs;
use std::io;

pub fn solve(input_file: &str){
    match part_1(input_file) {
        Ok(contents) => println!("Part 1: {:?}", contents),
        Err(e) => println!("Error in part 1: {}", e),
    }
    match part_2(input_file) {
        Ok(contents) => println!("Part 2: {}", contents),
        Err(e) => println!("Error in part 2: {}", e),
    }
}

// part 1 wants us to iterate over many vectors, and see how many of these vectors are both strictly monotonic 
// and "smooth", i.e. sequential values differ by 3 at most.
fn part_1(input_file: &str) -> io::Result<u32> {
    let reports = load_reports(input_file).unwrap();
    let mut safe: u32 = 0;
    for report in &reports{
        if is_sub_report_safe(report, None).unwrap() {
            safe += 1;
        } 
    }
    Ok(safe)
}

// part 2 wants us to repeat the same search, but this time, if a vector is not monotonic and
// smooth but removing any one value will make it so, then we will count it as monotonic and smooth. 
fn part_2(input_file: &str) -> io::Result<u32> {
    let reports = load_reports(input_file).unwrap();
    let mut safe: u32 = 0; 
    for report in &reports {
        if is_sub_report_safe(report, None).unwrap() {
            safe += 1;
        } else{
            for idx in 0..report.len() {
                if is_sub_report_safe(report, Some(idx)).unwrap() {
                    safe += 1;
                    break;
                }
            }
        }
    }
    Ok(safe)
}

fn is_sub_report_safe(report: &Vec<i32>, skipped_index: Option<usize>) -> io::Result<bool> {
    let mut level_safe: bool = true;
    let mut prev_diff: i32 = 0;
    let mut prev: Option<i32> = None;
    for (idx, &lvl) in report.iter().enumerate(){
        if Some(idx) == skipped_index {
            continue;
        }
        if let Some(p) = prev {
            let curr_diff: i32 = lvl - p;
            if curr_diff.abs() > 3 || curr_diff.abs() < 1{
                level_safe = false;
                break;
            } else if (curr_diff * prev_diff).is_negative(){
                level_safe = false;
                break;
            }
            prev_diff = curr_diff;
        }
        prev = Some(lvl);
    }
    Ok(level_safe)
}

fn load_reports(input_file: &str) -> io::Result<Vec<Vec<i32>>> {
    let content: String = fs::read_to_string(input_file)?;

    let mut all_reports = Vec::new();
    for line in content.lines() {
        let vu32: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        all_reports.push(vu32);
    }
    Ok(all_reports)
}
