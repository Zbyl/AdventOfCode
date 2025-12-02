use std::collections::HashSet;
use regex::Regex;
use crate::helpers::read_lines;

#[derive(Debug, Clone, Copy)]
struct Input {
    start: i64, // Inclusive.
    end: i64,   // Inclusive.
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Input>> {
    let mut inputs: Vec<Input> = Vec::new();
    if lines.len() != 1 {
        return Err(format!("Expected exactly one line of input, but got {}.", lines.len()).into());
    }
    let pieces = &lines[0].split(",").collect::<Vec<&str>>();

    let in_regex = Regex::new(r"^(?<start>\d+)-(?<end>\d+)$").unwrap();
    for (idx, piece) in pieces.iter().enumerate() {
        let cap = in_regex.captures(piece).ok_or(format!("Piece idx={} (zero-based) '{}' does not match the regex.", idx, piece))?;

        let input = Input {
            start: cap["start"].parse::<i64>().unwrap(),
            end: cap["end"].parse::<i64>().unwrap(),
        };
        inputs.push(input);
    }

    Ok(inputs)
}

#[allow(dead_code)]
fn find_invalids(mut input: Input) -> i64 {
    println!("============================================");
    println!("start={} end={}", input.start, input.end);

    // Find lowest number >= start, with even number of digits.
    let mut start_digits = input.start.ilog10() + 1;
    if (start_digits & 1) != 0 {
        input.start = 10i64.pow(start_digits);
        start_digits += 1;
    }

    let mut end_digits = input.end.ilog10() + 1;
    if (end_digits & 1) != 0 {
        end_digits -= 1;
        input.end = 10i64.pow(end_digits) - 1;
    }

    println!("start={} end={}", input.start, input.end);
    println!("start_digits={} end_digits={}", start_digits, end_digits);
    if end_digits == 0 {
        return 0;
    }

    // Find first and second half of start. Find maximum.
    let start_half_power = 10i64.pow(start_digits / 2);
    let start_high = input.start / start_half_power;
    let start_low = input.start % start_half_power;

    // Find first and second half of end. Find minimum.
    let end_half_power = 10i64.pow(end_digits / 2);
    let end_high = input.end / end_half_power;
    let end_low = input.end % end_half_power;

    println!("start_half_power={} start_high={} start_low={}", start_half_power, start_high, start_low);
    println!("end_half_power={} end_high={} end_low={}", end_half_power, end_high, end_low);

    if end_high < start_high {
        println!("result={}", 0);
        return 0;
    }

    let start_ok = if start_high < start_low { start_high + 1 } else { start_high };
    let end_ok = if end_high > end_low { end_high - 1 } else { end_high };

    let mut result = 0;
    let mut real_result = 0;
    for i in start_ok..=end_ok {
        result += i;
        let i_digits = i.ilog10() + 1;
        real_result += i * 10i64.pow(i_digits) + i;
    }

    println!("result={} real_result={}", result, real_result);
    return real_result;
}

#[allow(dead_code)]
fn solve_task(inputs: &Vec<Input>) -> i64 {
    let mut result: i64 = 0;
    for input in inputs {
        let res = find_invalids(input.clone());
        result += res;
    }
    return result;
}

fn decompose_num(mut num: i64, power: i64) -> Vec<i64> {
    let mut pieces = vec![];
    loop {
        if num == 0 {
            return pieces;
        }
        let piece = num % power;
        pieces.push(piece);
        num /= power;
    }
}

fn find_invalids2(mut input: Input, num_parts: u32, found_invalids: &mut HashSet<i64>) -> i64 {
    println!("============================================");
    println!("num_parts={} start={} end={}", num_parts, input.start, input.end);

    // Find lowest number >= start, with number of digits divisible by num_parts.
    let mut start_digits = input.start.ilog10() + 1;
    if (start_digits % num_parts) != 0 {
        start_digits = (start_digits / num_parts + 1) * num_parts;
        input.start = 10i64.pow(start_digits - 1);
    }

    // Find highest number < end, with number of digits divisible by num_parts.
    let mut end_digits = input.end.ilog10() + 1;
    if (end_digits % num_parts) != 0 {
        end_digits = (end_digits / num_parts) * num_parts;
        input.end = 10i64.pow(end_digits) - 1;
    }

    println!("start={} end={}", input.start, input.end);
    println!("start_digits={} end_digits={}", start_digits, end_digits);
    if end_digits == 0 {
        return 0;
    }

    // Find blocks of start. Find maximum.
    let start_block_power = 10i64.pow(start_digits / num_parts);
    let start_pieces = decompose_num(input.start, start_block_power);

    // Find blocks of end. Find minimum.
    let end_block_power = 10i64.pow(end_digits / num_parts);
    let end_pieces = decompose_num(input.end, end_block_power);

    println!("start_block_power={} start_pieces={:?}", start_block_power, start_pieces);
    println!("end_block_power={} end_pieces={:?}", end_block_power, end_pieces);

    let start_high = *start_pieces.last().unwrap();
    let mut start_ok = start_high;
    for &piece in start_pieces.iter().rev() {
        if piece < start_high {
            break;  // start_high is fine.
        }
        if piece > start_high {
            start_ok += 1;
            break;
        }
    }

    let end_high = *end_pieces.last().unwrap();
    let mut end_ok = end_high;
    for &piece in end_pieces.iter().rev() {
        if piece > end_high {
            break;  // end_high is fine.
        }
        if piece < end_high {
            end_ok -= 1;
            break;
        }
    }

    println!("start_ok={} end_ok={}", start_ok, end_ok);

    let mut result = 0;
    let mut real_result = 0;
    for i in start_ok..=end_ok {
        result += i;
        let i_digits = i.ilog10() + 1;
        let mut part_result = 0;
        for _k in 1..=num_parts {
            part_result = part_result * 10i64.pow(i_digits) + i;
        }

        if found_invalids.contains(&part_result) {
            println!("Skipping duplicate part_result={}", part_result);
            continue;
        }
        found_invalids.insert(part_result);

        real_result += part_result;
    }

    println!("result={} real_result={}", result, real_result);
    return real_result;
}

fn solve_task2(inputs: &Vec<Input>) -> i64 {
    let mut result: i64 = 0;
    let mut found_invalids: HashSet<i64> = HashSet::new();
    for input in inputs {
        let end_digits = input.end.ilog10() + 1;
        for block_size in 2..=end_digits {
            let res = crate::dec2::find_invalids2(input.clone(), block_size, &mut found_invalids);
            result += res;
        }
    }
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec2() {
    let lines = read_lines("dec2.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task(&inputs);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec2_2() {
    let lines = read_lines("dec2.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task2(&inputs);
    println!("{:?}", result);
}
