use std::collections::{HashMap, HashSet};
use crate::dec6::make_maze;
use crate::helpers::{read_lines, read_matrix_from_lines, separate_by_blank};

#[derive(Debug, Clone)]
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn parse_input(lines: &Vec<String>) -> Input {
    let (lines0, lines1) = separate_by_blank(lines);
    Input {
        patterns: lines0[0].split(',').map(|s| s.trim().to_string()).collect(),
        designs: lines1.iter().map(|s| s.trim().to_string()).collect(),
    }
}

fn compute_one_possible(patterns: &Vec<String>, design: &String) -> bool {
    let mut nodes = Vec::new();
    nodes.push(design.as_str());
    let mut visited = HashSet::new();

    loop {
        if nodes.is_empty() {
            return false;
        }

        let node = nodes.pop().unwrap();
        if node.is_empty() {
            return true;
        }

        for pattern in patterns.iter() {
            if let Some(stripped) = node.strip_prefix(pattern) {
                if !visited.contains(stripped) {
                    nodes.push(stripped);
                    visited.insert(stripped);
                }
            }
        }
    }
}

fn compute_possible(input: &Input) -> i64 {
    let mut result = 0;
    for design in &input.designs {
        if compute_one_possible(&input.patterns, design) {
            result += 1;
        }
    }
    result
}

fn compute_possible_count<'a>(patterns: &Vec<String>, design: &'a str, cache: &mut HashMap<&'a str, i64>) -> i64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(c) = cache.get(design) {
        return *c;
    }

    let mut count = 0;
    for pattern in patterns.iter() {
        if let Some(stripped) = design.strip_prefix(pattern) {
            count += compute_possible_count(&patterns, &stripped, cache);
        }
    }
    cache.insert(design, count);
    count
}

fn compute_possible2(input: &Input) -> i64 {
    let mut cache: HashMap<&str, i64> = HashMap::new();
    let mut result = 0;
    for design in &input.designs {
        result +=  compute_possible_count(&input.patterns, design.as_str(), &mut cache);
    }
    result
}

#[allow(dead_code)]
pub(crate) fn dec19() {
    let lines = read_lines("dec19.in.txt").expect("Could not load input.");
    let input = parse_input(&lines);
    let result = compute_possible(&input);
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec19_2() {
    let lines = read_lines("dec19.in.txt").expect("Could not load input.");
    let input = parse_input(&lines);
    let result = compute_possible2(&input);
    println!("{:?}", result);
}
