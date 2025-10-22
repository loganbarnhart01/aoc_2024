use std::fs;
use std::io;
use std::iter::zip;
use std::collections::HashMap;

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

// The whole idea behind part 1 is to take two lists and compute 
// \sum |list1_i - list2_i| after the elements are sorted
fn part_1() -> io::Result<u32> {
    let (list_one, list_two) = load_sorted_lists()?;
    let mut distances = Vec::new();
    for (n1, n2) in zip(list_one, list_two){
        distances.push(n1.abs_diff(n2));
    }
    Ok(distances.iter().sum())
}

// Part 2 wants us to use the same two lists and compute:
// \sum list1_i * n_i where n_i := the number of times list1_i
// appears in list2
fn part_2() -> io::Result<u32>{
    let (list_one, list_two) = load_sorted_lists()?;
    let mut frequencies: HashMap<u32, u32> = HashMap::new();

    // rust does something interesting, pattern matching -> auto deref. 
    // The below loop is the same as:
    // for n2 in &list_two {
    //     let num = *n2;  
    // so, we don't need to manually dereference
    // 
    // You may ask, why are we even bothering with iterating over &list_two below?
    // Well, we don't need to for this problem, but if we were to iterate over
    // list_two instead, it would be consumed + unaccessable later. So we're just practicing.
    
    for &n2 in &list_two{ 
        *frequencies.entry(n2).or_insert(0) += 1;
    }
    let mut similarity = 0;
    for &n1 in &list_one {
        let count = frequencies.get(&n1).unwrap_or(&0); // need to pass a reference for 0 here,
                                                        // since if n1 is a key in the table .get
                                                        // returns &value1, we can't return &u32 if
                                                        // key is present but a u32 if it's not
                                                        // present
        similarity += n1*count;
    }
    Ok(similarity)
}

fn load_sorted_lists() -> io::Result<(Vec<u32>, Vec<u32>)> {
    let content: String = fs::read_to_string("./inputs/day01_p1.txt")?;

    let mut list_one = Vec::new();
    let mut list_two = Vec::new();
    for line in content.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        list_one.push(v[0].parse::<u32>().unwrap());
        list_two.push(v[1].parse::<u32>().unwrap());
    }
    list_one.sort();
    list_two.sort();
    Ok((list_one, list_two))
}
