use std::fs;
use std::io;

pub fn solve(input_file: &String) {
    match part_1(input_file) {
        Ok(contents) => println!("Part 1: {:?}", contents),
        Err(e) => println!("Error in part 1: {}", e),
    }
    match part_2() {
        Ok(contents) => println!("Part 2: {:?}", contents),
        Err(e) => println!("Error in part 1: {}", e),
    }
}

fn part_1(input_file: &str) -> io::Result<u32> {
    let input = load_input_chars(input_file).unwrap();
    println!("{:?}", input[0][3]);
    let mut xmas = 0;
    for row_idx in 0..input.len() {
        for col_idx in 0..input[0].len() {
            xmas += check_words_at(&input, row_idx, col_idx).unwrap(); 
        }
    }
    Ok(xmas)   
}

// I learned that I have python brain and am taking indexing values for granted. 
// The following line doesn't work as intended! 
//
// let word = &mat[row_idx..row_idx+4][col_idx]; 
//
// The intent is to get the 4 characters in column col_idx below row_idx.
//
// Instead, mat[row_idx..row_idx+4] returns a slice of rows from row_idx..row_idx + 4, then we select col_idx
// from those. 

fn check_words_at(mat: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> io::Result<u32> {
    let fwd = ['X', 'M', 'A', 'S'];
    let bwd = ['S', 'A', 'M', 'X'];
    let search_row = mat.len() - col_idx >= 4;
    let search_down = mat[0].len() - row_idx >= 4;
    let search_up = row_idx >= 3;

    let mut found = 0;
    if search_row {
        let horz: Vec<char> = (0..4)
            .map(|ii| mat[row_idx][col_idx + ii])
            .collect();
        if horz == fwd { found += 1 }
        else if horz == bwd { found += 1 }
    }
    if search_down {
        let vert: Vec<char> = (0..4)
            .map(|ii| mat[row_idx + ii][col_idx])
            .collect();
        if vert == &fwd { found += 1 }
        else if vert == &bwd { found += 1 }
    }
    if search_row && search_down { // look for diag down right 
        let diag: Vec<char> = (0..4)
            .map(|offset| mat[row_idx + offset][col_idx + offset])
            .collect();
        if diag == fwd { found += 1 }
        if diag == bwd { found += 1 }
    }
    if search_row && search_up {   // look for diag up right 
        let diag: Vec<char> = (0..4)
            .map(|offset| mat[row_idx - offset][col_idx + offset])
            .collect();
        if diag == fwd { found += 1 }
        if diag == bwd { found += 1 }
    }
    Ok(found)
}

fn part_2() -> io::Result<u32> {
    Ok(1)   
}

fn load_input_chars(input_file: &str) -> io::Result<Vec<Vec<char>>> {
    let content: String = fs::read_to_string(input_file)?;
    let mut lines = Vec::new();
    for line in content.lines() {
        lines.push(line.chars().collect());
    }
    Ok(lines)
}
