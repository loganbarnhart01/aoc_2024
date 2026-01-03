use std::fs;

pub fn solve(input_file: &String) {
    println!("Part 1: {:?}", part_1(input_file));
    println!("Part 2: {}", part_2(input_file));
}

fn part_1(input_file: &str) -> u32 {
    let input = load_input_chars(input_file);
    let mut xmas = 0;
    for row_idx in 0..input.len() {
        for col_idx in 0..input[0].len() {
            xmas += check_word_at(&input, row_idx, col_idx, Direction::Right, "XMAS");
            xmas += check_word_at(&input, row_idx, col_idx, Direction::Right, "SAMX");

            xmas += check_word_at(&input, row_idx, col_idx, Direction::Down, "XMAS");
            xmas += check_word_at(&input, row_idx, col_idx, Direction::Down, "SAMX");

            xmas += check_word_at(&input, row_idx, col_idx, Direction::DRight, "XMAS");
            xmas += check_word_at(&input, row_idx, col_idx, Direction::DRight, "SAMX");

            xmas += check_word_at(&input, row_idx, col_idx, Direction::URight, "XMAS");
            xmas += check_word_at(&input, row_idx, col_idx, Direction::URight, "SAMX");
        }
    }
    xmas   
}

fn part_2(input_file: &str) -> u32 {
    let input = load_input_chars(input_file);
    let mut x = 0;
    for row_idx in 0..input.len()-2 {
        for col_idx in 0..input[0].len() {
            let mut mas = 0;
            let mut sam = 0;
            mas += check_word_at(&input, row_idx, col_idx, Direction::DRight, "MAS");
            sam += check_word_at(&input, row_idx, col_idx, Direction::DRight, "SAM");

            mas += check_word_at(&input, row_idx + 2, col_idx, Direction::URight, "MAS");
            sam += check_word_at(&input, row_idx + 2, col_idx, Direction::URight, "SAM");

            if mas + sam >= 2 { x  += 1 }
        }
    }
    x
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    URight,
    DRight,
}

fn check_word_at(mat: &Vec<Vec<char>>, row_idx: usize, col_idx: usize, direction: Direction, word: &str) -> u32 {
    let search: bool;
    let mut row_mult: i32 = 0;
    let mut col_mult: usize = 0;
    match direction {
        Direction::Right => { 
            search = mat.len() - col_idx >= word.len(); 
            col_mult = 1;
        },
        Direction::Down => { 
            search = mat[0].len() - row_idx >= word.len(); 
            row_mult = 1;
        },
        Direction::DRight => { 
            search = mat[0].len() - row_idx >= word.len() && mat.len() - col_idx >= word.len(); 
            row_mult = 1;
            col_mult = 1;
        },
        Direction::URight => { 
            search = row_idx >= word.len()-1 && mat.len() - col_idx >= word.len(); 
            col_mult = 1;
            row_mult = -1;
        },
    }

    if search {
        let word_present: bool = &(0..word.len())
            .map(|ii| mat[usize::try_from(i32::try_from(row_idx).unwrap() + row_mult * i32::try_from(ii).unwrap()).unwrap()][col_idx + col_mult * ii])
            .collect::<String>() == word;
        return u32::from(word_present)
    }
    0
}


fn load_input_chars(input_file: &str) -> Vec<Vec<char>> {
    let content: String = fs::read_to_string(input_file).unwrap();
    let mut lines = Vec::new();
    for line in content.lines() {
        lines.push(line.chars().collect());
    }
    lines
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
