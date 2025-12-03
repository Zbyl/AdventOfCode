use regex::Regex;
use crate::basic_parsing::read_lines;

#[derive(Debug, Clone, Copy)]
struct Input {
    left: bool,
    num: i64,
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Input>> {
    let mut inputs: Vec<Input> = Vec::new();

    let in_regex = Regex::new(r"^(?<dir>[LR])(?<num>\d+)$").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        let cap = in_regex.captures(line).ok_or(format!("Line idx={} (zero-based) '{}' does not match the regex.", idx, line))?;

        let input = Input {
            left: &cap["dir"] == "L",
            num: cap["num"].parse::<i64>().unwrap(),
        };
        inputs.push(input);
    }

    Ok(inputs)
}

fn compute_stuff(inputs: &Vec<Input>, start_pos: i64) -> i64 {
    let mut current_pos = start_pos;
    let mut count: i64 = if start_pos == 0 { 1 } else { 0 };
    for input in inputs {
        current_pos += if input.left { -input.num } else { input.num };
        current_pos %= 100;
        if current_pos < 0 {
            current_pos += 100;
        }

        if current_pos == 0 {
            count += 1;
        }
    }
    return count;
}

fn compute_stuff2(inputs: &Vec<Input>, start_pos: i64) -> i64 {
    let mut current_pos = start_pos;
    let mut count: i64 = if start_pos == 0 { 1 } else { 0 };
    for input in inputs {
        if input.num == 0 {
            continue;
        }

        let pre_pos = current_pos;

        if input.left {
            current_pos = -current_pos;
        }

        let rotations = input.num / 100;
        let delta = input.num % 100;

        current_pos += delta + 100;
        current_pos %= 100;
        if current_pos < 0 { panic!("Impossible") }

        if input.left {
            current_pos = (100 - current_pos) % 100;
        }
        if current_pos < 0 { panic!("Impossible") }

        count += rotations;
        if pre_pos == 0 {
            if current_pos == 0 { panic!("Impossible") }
            continue;
        }

        if current_pos == 0 {
            count += 1;
            continue;
        }

        if input.left && (current_pos > pre_pos) {
            count += 1;
        }
        if !input.left && (current_pos < pre_pos) {
            count += 1;
        }
    }
    return count; // 6583 too low
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec1() {
    let lines = read_lines("dec1.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = compute_stuff(&inputs, 50);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec1_2() {
    let lines = read_lines("dec1.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = compute_stuff2(&inputs, 50);
    println!("{:?}", result);
}
