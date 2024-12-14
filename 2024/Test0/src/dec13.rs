use regex::Regex;
use crate::helpers::{read_lines, Vec2};

#[derive(Debug, Clone, Copy)]
struct Input {
    a: Vec2<i64>,
    b: Vec2<i64>,
    prize: Vec2<i64>,
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Input>> {
    let mut inputs: Vec<Input> = Vec::new();
    //return Err(format!("Line idx={} (zero-based) {} does not match the rule regex.", 0, 1).into());

    let button_regex = Regex::new(r"^Button (?:A|B): X\+(?<x>\d+), Y\+(?<y>\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(?<x>\d+), Y=(?<y>\d+)$").unwrap();
    let mut idx = 0;
    while idx < lines.len() {
        let line0 = lines.get(idx).unwrap();
        if line0.is_empty() {
            idx += 1;
            continue;
        }

        let line1 = lines.get(idx + 1).unwrap();
        let line2 = lines.get(idx + 2).unwrap();
        let button_a = button_regex.captures(line0).ok_or(format!("Line idx={} (zero-based) '{}' does not match the A button regex.", idx, line0))?;
        let button_b = button_regex.captures(line1).ok_or(format!("Line idx={} (zero-based) '{}' does not match the B button regex.", idx + 1, line1))?;
        let prize = prize_regex.captures(line2).ok_or(format!("Line idx={} (zero-based) '{}' does not match the prize regex.", idx + 2, line2))?;
        idx += 3;

        let input = Input {
            a: Vec2::<i64>::new(button_a["x"].parse::<i64>().unwrap(), button_a["y"].parse::<i64>().unwrap()),
            b: Vec2::<i64>::new(button_b["x"].parse::<i64>().unwrap(), button_b["y"].parse::<i64>().unwrap()),
            prize: Vec2::<i64>::new(prize["x"].parse::<i64>().unwrap(), prize["y"].parse::<i64>().unwrap()),
        };
        inputs.push(input);
    }

    Ok(inputs)
}

/*
    3a + b = money

    AXa + BXb = PX
    AYa + BYb = PY

    a = (PX - BXb) / AX
    AYa + BYb = PY

    a = (PX - BXb) / AX
    AY ((PX - BXb) / AX) + BYb = PY
    AY/AX * PX - AY/AX * BXb + BYb = PY
    AY/AX * PX - PY = b (AY/AX * BX - BY)
    b = (AY/AX * PX - PY) / (AY/AX * BX - BY)
    b = (AY * PX - AX * PY) / (AY * BX - BY * AX)
    a = (PX - BX)/AX * (AY * PX - PY * AX) / (AY * BX - BY * AX)
    a = PX * AY * PX - PX * PY * AX - BX * AY * PX + BX * PY * AX // (AY * BX - BY * AX) AX
    a = (BY * PX - BX * PY) / (AY * BX - BY * AX)
 */

fn check(input: &Input, a: i64, b: i64) -> bool {
    (input.a * a + input.b * b) == input.prize
}

fn pay(a: i64, b: i64) -> i64 {
    3 * a + b
}

fn compute_cost(input: &Input) -> Option<i64> {
    if (input.a.x == 0) && (input.b.x == 0) {
        if check(input, 0, 0) { return Some(pay(0, 0)) }
        return None;
    }

    if input.a.x == 0 {
        let cb = input.prize.x / input.b.x;
        if check(input, 0, cb) { return Some(pay(0, cb)) }

        if (input.a.y == 0) { return None; };

        let ca = (input.prize.y - cb * input.b.y) / input.a.y;
        if check(input, ca, cb) { return Some(pay(ca, cb)) }

        return None;
    }

    if input.b.x == 0 {
        let ca = input.prize.x / input.a.x;
        if check(input, ca, 0) { return Some(pay(ca, 0)) }

        if (input.b.y == 0) { return None; };

        let cb = (input.prize.y - ca * input.a.y) / input.b.y;
        if check(input, ca, cb) { return Some(pay(ca, cb)) }

        return None;
    }

    if input.a.y == 0 {
        let cb = input.prize.y / input.b.y;
        if check(input, 0, cb) { return Some(pay(0, cb)) }

        let ca = (input.prize.x - cb * input.b.x) / input.a.x;
        if check(input, ca, cb) { return Some(pay(ca, cb)) }

        return None;
    }

    if input.b.y == 0 {
        let ca = input.prize.y / input.a.y;
        if check(input, ca, 0) { return Some(pay(ca, 0)) }

        let cb = (input.prize.x - ca * input.a.x) / input.b.x;
        if check(input, ca, cb) { return Some(pay(ca, cb)) }

        return None;
    }

    let den = input.b.x * input.a.y - input.b.y * input.a.x;
    if den == 0 {
        return None;
    }

    let nx = input.b.x * input.prize.y - input.b.y * input.prize.x;
    let ny = input.a.y * input.prize.x - input.a.x * input.prize.y;
    let ca = nx / den;
    let cb = ny / den;

    if check(input, ca, cb) { return Some(pay(ca, cb)) }
    None
}

fn compute_costs(inputs: &Vec<Input>) -> i64 {
    let mut cost = 0;
    for input in inputs.iter() {
        cost += compute_cost(input).unwrap_or(0);
    }
    cost
}

#[allow(dead_code)]
pub(crate) fn dec13() {
    let lines = read_lines("dec13.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = compute_costs(&inputs);
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec13_2() {
    let lines = read_lines("dec13.in.txt").expect("Could not load input.");
    let mut inputs = parse_input(&lines).unwrap();
    for input in inputs.iter_mut() {
        input.prize += Vec2::<i64>::new(10000000000000, 10000000000000);
    }
    let result = compute_costs(&inputs);
    println!("{:?}", result);
}
