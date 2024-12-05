use itertools::Itertools;
use regex::Regex;
use crate::dec4::{read_lines};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct Dec5Input {
    rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

pub(crate) fn read_dec5_input(filename: &str) -> crate::dec4::Result<Dec5Input> {
    let lines = read_lines(filename)?;

    let mut input = Dec5Input { rules: Vec::new(), updates: Vec::new() };

    let rule_regex = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    let mut reading_rules = true;
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let Some(caps) = rule_regex.captures(line) else {
                return Err(format!("Line idx={} (zero-based) {} does not match the rule regex.", idx, line).into());
            };
            let rule = caps.iter().skip(1)
                .map(|c| c.unwrap().as_str().parse::<i32>().expect(format!("Line idx={} (zero-based) {}: cannot parse {:?} as int.", idx, line, c).as_str()))
                .collect_tuple().unwrap();
            input.rules.push(rule);
        } else {
            let pieces = line.split(",")
                .map(|e| e.parse::<i32>().expect(format!("Line idx={} (zero-based) {}: cannot parse {} as int.", idx, line, e).as_str()))
                .collect();
            input.updates.push(pieces);
        }
    }

    Ok(input)
}

fn make_rule_map(rules: &Vec<(i32, i32)>) -> HashMap<i32, HashSet<i32>> {
    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (a, b) in rules {
        if rule_map.contains_key(a) {
            rule_map.get_mut(a).unwrap().insert(*b);
        } else {
            rule_map.insert(*a, HashSet::from([*b]));
        }
    }

    return rule_map;
}

fn is_update_ok(update: &Vec<i32>, rule_map: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut prevs = HashSet::new();
    let empty = HashSet::new();
    for a in update {
        let must_follow = rule_map.get(a).unwrap_or(&empty);
        if prevs.intersection(must_follow).next().is_some() {
            println!("Update {:?} number {} must be followed by {:?} but some are in previous {:?}", update, a, must_follow, prevs);
            return false;
        }
        prevs.insert(*a);
    }
    true
}

fn fix_update(update: &Vec<i32>, rule_map: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let empty = HashSet::new();
    let mut visited = HashSet::new();
    let updateH = update.iter().cloned().collect::<HashSet<_>>();
    let mut path = Vec::new();
    let mut order = Vec::new();

    for top_node in update {
        if !visited.contains(top_node) {
            path.push(*top_node);
            visited.insert(*top_node);
            println!("top {top_node}");
        }

        while !path.is_empty() {
            let node = *path.last().unwrap();

            let mut added = false;
            let must_follow = rule_map.get(&node).unwrap_or(&empty);
            let children = must_follow.intersection(&updateH).collect_vec();
            for child in children {
                if !visited.contains(&child) {
                    path.push(*child);
                    visited.insert(*child);
                    added = true;
                    print!("{child} ");
                    break;
                }
            }

            if !added {
                path.pop();
                order.push(node);
                println!("-> {node}");
            } else {
                println!("");
            }
        }
    }

    order.reverse();
    order
}


fn process_updates(input: &Dec5Input, rule_map: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut result = 0;
    for update in &input.updates {
        if !is_update_ok(update, rule_map) {
            continue;
        }
        if update.len() % 2 == 0 {
            panic!("Update {:?} is not odd length.", update);
        }
        result += update[update.len() / 2];
    }
    return result;
}

fn process_updates2(input: &Dec5Input, rule_map: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut result = 0;
    for update in &input.updates {
        if is_update_ok(update, rule_map) {
            continue;
        }

        if update.len() % 2 == 0 {
            panic!("Update {:?} is not odd length.", update);
        }
        let fixed_update = fix_update(update, rule_map);
        println!("Fixed: {:?}", fixed_update);

        result += fixed_update[fixed_update.len() / 2];
    }
    return result;
}

#[allow(dead_code)]
pub(crate) fn dec5() {
    let input = read_dec5_input("dec5.in.txt").expect("Could not load input.");
    let rule_map = make_rule_map(&input.rules);
    let result = process_updates(&input, &rule_map);
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec5_2() {
    let input = read_dec5_input("dec5.in.txt").expect("Could not load input.");
    let rule_map = make_rule_map(&input.rules);
    let result = process_updates2(&input, &rule_map);
    println!("{:?}", result);
}
