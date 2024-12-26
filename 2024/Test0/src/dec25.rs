use crate::dec24::Op;
use crate::helpers::read_lines;

type Comb = [i8; 5];

fn is_match(key: &Comb, lock: &Comb) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 { return false; }
    }
    true
}

#[derive(Debug, Clone)]
struct Input {
    keys: Vec<Comb>,
    locks: Vec<Comb>,
}

fn parse_input(lines: &Vec<String>) -> Input {
    let mut input = Input {
        keys: Vec::new(),
        locks: Vec::new(),
    };

    let mut idx = 0;
    loop {
        if idx >= lines.len() { return input; }

        let line = lines.get(idx).unwrap();
        idx += 1;
        if line.is_empty() { continue; }
        let is_key = line == ".....";

        let mut comb: Comb = [0; 5];
        for i in 0..5 {
            let line = lines.get(idx).unwrap();
            idx += 1;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    comb[j] += 1;
                }
            }
        }

        let line = lines.get(idx).unwrap();
        idx += 1;
        let exp = (if is_key { "#####" } else { "....." });
        if line != exp {
            panic!("Expected {} but got {}", exp, line);
        }

        (if is_key { &mut input.keys } else { &mut input.locks }).push(comb);
    }
}

fn brute_match(input: &Input) -> i32 {
    let mut result = 0;
    for key in &input.keys {
        for lock in &input.locks {
            if is_match(key, lock) {
                result += 1;
            }
        }
    }
    result
}

#[allow(dead_code)]
pub(crate) fn dec25() {
    let lines = read_lines("dec25.in.txt").expect("Could not load input.");
    let input = parse_input(&lines);
    let result = brute_match(&input);
    println!("{:?}", result);
}
