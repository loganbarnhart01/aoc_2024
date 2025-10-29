use std::fs;
use std::io;

pub fn solve(){
    match part_1() {
        Ok(contents) => println!("Part 1: {:?}", contents),
        Err(e) => println!("Error in part 1: {}", e),
    }
    match part_2() {
        Ok(contents) => println!("Part 2: {}", contents),
        Err(e) => println!("Error in part 2: {}", e),
    }
}

// part 1 wants us to iterate over many vectors, and see how many of these vectors are both strictly monotonic 
// and "smooth", i.e. sequential values differ by 3 at most.
fn part_1() -> io::Result<u32> {
    let reports = load_reports().unwrap();
    let mut safe: u32 = 0;
    for report in &reports{
        let mut level_safe: bool = true;
        let mut prev_diff: i32 = 0;
        for (idx, &lvl) in report[1..].iter().enumerate(){
            let curr_diff: i32 = lvl - report[idx];
            if curr_diff.abs() > 3 || curr_diff.abs() < 1{
                level_safe = false;
                break;
            } else if (curr_diff * prev_diff).is_negative(){
                level_safe = false;
                break;
            }
            prev_diff = curr_diff;
        }
        if level_safe {safe += 1}
    }
    Ok(safe)
}

// part 2 wants us to repeat the same search, but this time, if a vector is not monotonic and
// smooth but removing any one value will make it so, then we will count it as monotonic and smooth. 
fn part_2() -> io::Result<u32> {
    Ok(2)
}

fn load_reports() -> io::Result<Vec<Vec<i32>>> {
    let content: String = fs::read_to_string("./inputs/day02_p1.txt")?;

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
