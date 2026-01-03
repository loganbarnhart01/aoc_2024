use std::fs;
use std::collections::HashMap;
use std::ops::Div;

pub fn solve(input_file: &String) {
    println!("Part 1: {:?}", part_1(input_file));
    println!("Part 2: {}", part_2(input_file));
}

fn part_1(input_file: &str) -> usize {
    let (rules, manuals) = load_rules(input_file);
    let mut middles: Vec<usize> = Vec::new();

    // Make hashmap of {number : [all, numbers, that, need, to, be, after]}
    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in &rules {
        rule_map.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
    }

    for manual in manuals {
        let mut valid: bool = true;
        // We really just need to see that all entries before the current one don't break a rule
        // so iterate in reverse order + check all prior entries for rule violation
        for idx in (0..(manual.len())).rev() {
            // get current entry, prior entries, and rules for current
            let curr = manual[idx];
            let before = &manual[0..idx];
            let rule = match rule_map.get(&curr) {
                Some(value) => value,
                None => &vec![],
            };
            // see if any prior break a rule. quit early if so.
            for prev in before {
                for candidate in rule {
                    if candidate == prev {
                        valid = false;
                    }
                    if valid == false { break; }
                }
                if valid == false { break; }
            }
        }
        // save middle value if no rules broken
        if valid == true { 
            let middle = manual[manual.len().div(2)];
            middles.push(middle); 
        }
    }
    middles.iter().sum()
}

fn part_2(input_file: &str) -> usize {
    let (rules, manuals) = load_rules(input_file);
    let mut middles: Vec<usize> = Vec::new();
    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in &rules {
        rule_map.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
    }
    for manual in manuals {
        let mut valid: bool = true;
        for idx in (0..(manual.len())).rev() {
            let curr = manual[idx];
            let before = &manual[0..idx];
            let rule = match rule_map.get(&curr) {
                Some(value) => value,
                None => &vec![],
            };
            // reorder if a rule is broken:
            for prev in before {
                for candidate in rule {
                    if candidate == prev {
                        valid = false;
                    }
                    if valid == false { break; }
                }
                if valid == false { break; }
            }
        }

        if valid == false { 
            let middle = get_ordered_middle(&manual, &rule_map).unwrap();
            middles.push(middle)
        }
    }
    middles.iter().sum()
}

// Gets the middle entry from the ordered updates
// We don't need to actually order the updates, we just need to find the first few until we reach
// the middle update. 
// 
// We can tell that an entry is next in the order if it doesn't appear in any of the rules of the 
// other entries
fn get_ordered_middle(manual: &Vec<usize>, rule_map: &HashMap<usize, Vec<usize>>) -> Result<usize, String> {
    let mid_idx = manual.len().div(2);
    let mut n_ordered: usize = 0;
    let mut possible_middles: HashMap<usize, bool> = HashMap::new();
    for entry in manual {
        possible_middles.insert(*entry, true);
    }
    for _ in 0..(mid_idx + 1){
        for entry in manual {
            let mut curr_next: bool = true;
            if *possible_middles.get(entry).unwrap() {
                // Get all other entries in the manual that're potential middles and not the current entry
                let others: Vec<&usize> = manual[0..manual.len()]
                    .iter()
                    .filter(|&v| (*possible_middles.get(v).unwrap()) && (v != entry))
                    .collect();
                // iterate over all rules + continue to next entry if current is in a rule 
                // otherwise update as not a potential middle or return if we've ordered enough
                for other in others {
                    let other_rule: &Vec<usize> = match rule_map.get(other) {
                        Some(rule) => rule,
                        None => &vec![],
                    };
                    for candidate in other_rule {
                        if candidate == entry { curr_next = false; }
                        if curr_next == false { break; }
                    }
                    if curr_next == false { break; }
                }
                if curr_next == true { 
                    n_ordered += 1;
                    if n_ordered == mid_idx + 1 { 
                        return Ok(*entry) 
                    }
                    else { possible_middles.insert(*entry, false); }
                }
            }
        }
    }
    Err(String::from("Invalid manual, unable to reorder properly."))
}


// Return a tuple of arrays. 
//   First array contains arrays of rules like [before, after]
//   Second array contains arrays of the manuals
fn load_rules(input_file: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let content: String = fs::read_to_string(input_file).unwrap();
    let mut rules = Vec::new();
    let mut manuals = Vec::new();

    let mut on_rules: bool = true;
    for line in content.lines() {
        if line == "" { 
            on_rules = false; 
            continue;
        }
        if on_rules == true { 
            let split = line
                        .split('|')
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
            rules.push(split); 
        }
        if on_rules == false { 
            let split = line
                        .split(',')
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
            manuals.push(split); }
    }
    (rules, manuals)
}
