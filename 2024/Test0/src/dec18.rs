use std::collections::{HashMap, HashSet};
use regex::Regex;
use crate::dec6::Direction;
use crate::helpers::{read_lines, Matrix, Vec2};

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Vec2>> {
    let mut result: Vec<Vec2> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let err = format!("Line idx={} (zero-based) '{}' does not match the point format.", idx, line);
        let (x, y) = line.split_once(',').ok_or(err.clone())?;
        result.push(Vec2::new(x.parse()?, y.parse()?));
    }

    Ok(result)
}

fn best_path(matrix: &Matrix, bad_ones: &[Vec2], start: Vec2, end: Vec2) -> Option<i64> {
    let mut matrix = matrix.clone();
    for &bad in bad_ones {
        matrix.put(bad, '#');
    }
    println!("{}", bad_ones.len());
    //println!("{}", matrix);

    let mut wavefront = Vec::<Vec2>::new();
    let mut best_dists: HashMap<Vec2, i64> = HashMap::new();
    wavefront.push(start);
    best_dists.insert(start, 0);
    let mut idx = 0;

    loop {
        if idx >= wavefront.len() {
            return None; //panic!("No path to end found.");
        }

        let pos = wavefront[idx];
        idx += 1;
        let &dist = best_dists.get(&pos).unwrap();

        if pos == end { return Some(dist); }

        for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let next_pos = pos + dir.dir();
            if best_dists.contains_key(&next_pos) { continue; }
            let val = matrix.get(next_pos);
            if val == None { continue; }
            if val.unwrap() == '#' { continue; }
            wavefront.push(next_pos);
            best_dists.insert(next_pos, dist + 1);
        }
    }
}

fn find_blocker(matrix: &Matrix, bad_ones: &[Vec2], start: Vec2, end: Vec2) -> Option<Vec2> {
    for idx in 0..bad_ones.len() {
        let result = best_path(&matrix, &bad_ones[0..idx + 1], start, end);
        if result.is_none() { return Some(bad_ones[idx]); }
    }
    None
}

#[allow(dead_code)]
pub(crate) fn dec18() {
    let lines = read_lines("dec18.in.txt").expect("Could not load input.");
    let mut input = parse_input(&lines).unwrap();
    let matrix = Matrix::new(71, 71, '.', None);
    println!("{}", matrix);
    input.split_off(1024);
    let result = best_path(&matrix, &input, Vec2::new(0, 0), Vec2::new(matrix.width as i32 - 1, matrix.height as i32 - 1));
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec18_2() {
    let lines = read_lines("dec18.in.txt").expect("Could not load input.");
    let input = parse_input(&lines).unwrap();
    let matrix = Matrix::new(71, 71, '.', None);
    println!("{}", matrix);
    let result = find_blocker(&matrix, &*input, Vec2::new(0, 0), Vec2::new(matrix.width as i32 - 1, matrix.height as i32 - 1));
    println!("{:?}", result);
}
