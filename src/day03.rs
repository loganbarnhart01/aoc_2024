use std::fs;


pub fn solve(input_file: &str) {
    println!("Part 1: {:?}", part_1(input_file));
    println!("Part 2: {}", part_2(input_file));
}

fn part_1(input_file: &str) -> u32 {
    let input: Vec<char> = load_input_chars(input_file);
    if input.len() < 8 { return 0 }
    let mut result: u32 = 0; 
    let mut head: usize = 0;
    let mut tail: usize = 0;
    while head + 4 < input.len() && tail < input.len(){
        if input[head..head+4] == ['m', 'u', 'l', '('] { // look for valid start first
            tail = head + 4;  
            let mut valid_mul: bool = true;
            let mut comma_idx: usize = 0;                // keep track of comma for parsing
            while tail < input.len() {
                tail += 1;
                if input[tail] == ')' { 
                    break;
                } else if input[tail] != ',' && !input[tail].is_numeric() {
                    valid_mul = false;
                    break;
                } else if input[tail] == ',' {
                    comma_idx = tail;
                }
            }
            if valid_mul {
                let mult1: u32 = cast_chars_to_int(&input[head+4..comma_idx]);
                let mult2: u32 = cast_chars_to_int(&input[comma_idx+1..tail]);
                result += mult1 * mult2;
            }
            head = tail;
        } else {
            head += 1;
        }
    }
    result
}

fn part_2(input_file: &str) -> u32 {
    let input: Vec<char> = load_input_chars(input_file);
    if input.len() < 8 { return 0 }
    let mut result: u32 = 0; 
    let mut head: usize = 0;
    let mut tail: usize = 0;
    let mut do_on: u32 = 1;
    while head + 7 < input.len() && tail < input.len(){
        if input[head..head+4] == ['d', 'o', '(', ')'] { // look for valid start first
            do_on = 1;
            head = head + 4;
        } else if input[head..head+7] == ['d', 'o', 'n', '\'', 't', '(', ')'] { // look for valid start first
            do_on = 0;
            head = head + 6;
        } else if input[head..head+4] == ['m', 'u', 'l', '('] { // look for valid start first
            tail = head + 4;  
            let mut valid_mul: bool = true;
            let mut comma_idx: usize = 0;                // keep track of comma for parsing
            while tail < input.len() {
                tail += 1;
                if input[tail] == ')' { 
                    break;
                } else if input[tail] != ',' && !input[tail].is_numeric() {
                    valid_mul = false;
                    break;
                } else if input[tail] == ',' {
                    comma_idx = tail;
                }
            }
            if valid_mul {
                let mult1: u32 = cast_chars_to_int(&input[head+4..comma_idx]);
                let mult2: u32 = cast_chars_to_int(&input[comma_idx+1..tail]);
                result += do_on * mult1 * mult2;
            }
            head = tail;
        } else {
            head += 1;
        }
    }
    result
}

fn cast_chars_to_int(characters: &[char]) -> u32 {
    let mut len: u32 = characters.len().try_into().unwrap();
    let mut result: u32 = 0;
    let radix: u32 = 10;
    for ch in characters{
        len -= 1;
        result += radix.pow(len) * ch.to_digit(10).unwrap();
    }
    result
}


// Interesting note about string slicing / substrings: 
// Indexing can only be done with vars of type 'usize'. This means the returned data is a
// slice of *bytes*. So slicing like my_str[..n] will return if my_str is ascii characters
// since each char is of size usize bytes. But other alphabets will return potentially
// fewer than n letters
// two options for arbitrary indexing of the string. Either we can load it as a vector of
// individual characters, or we can convert the string to bytes so we know the exact size of
// data to expect. 
//
// Loading as bytes requires `my_bytes[idx] as char` to do character comparisons later

// fn load_input_bytes() -> Vec<u8> {
//     let content: String = fs::read_to_string("./inputs/day02_p1.txt")?;
//     content.into_bytes()
// }

fn load_input_chars(input_file: &str) -> Vec<char> {
    let content: String = fs::read_to_string(input_file).unwrap();
    content.chars().collect()
}
