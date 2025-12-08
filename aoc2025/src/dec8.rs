use std::ops;
use itertools::Itertools;
use regex::Regex;
use crate::basic_parsing::read_lines;
use crate::find_union::FindUnion;
use crate::vec2::Vec2;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[allow(dead_code)]
impl Vec3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub const fn len2(&self) -> i64 {
        return self.x * self.x + self.y * self.y +self.z * self.z;
    }
}

impl ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Vec3>> {
    let mut inputs: Vec<Vec3> = Vec::new();
    let in_regex = Regex::new(r"^(?<x>\d+),(?<y>\d+),(?<z>\d+)$").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        let cap = in_regex.captures(line).ok_or(format!("Line idx={} (zero-based) '{}' does not match the regex.", idx, line))?;

        let input = Vec3 {
            x: cap["x"].parse::<i64>().unwrap(),
            y: cap["y"].parse::<i64>().unwrap(),
            z: cap["z"].parse::<i64>().unwrap(),
        };
        inputs.push(input);
    }

    Ok(inputs)
}

#[allow(dead_code)]
fn solve_task(inputs: &Vec<Vec3>, num_joins: i32) -> i64 {
    let mut pair_distances = vec![];

    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            let dist = (inputs[j] - inputs[i]).len2();
            pair_distances.push((Vec2::new(i as i64, j as i64), dist));
        }
    }

    pair_distances.sort_by_key(|item| item.1);

    let mut circuits = FindUnion::new();
    for i in 0..num_joins {
        let (Vec2 {x, y}, _dist) = pair_distances[i as usize];
        circuits.join(x, y);
    }

    let sets = circuits.get_sets();
    let result = sets.iter().map(|s| s.len() as i64).sorted().rev().take(3).product();
    return result;
    // not 36
}

#[allow(dead_code)]
fn solve_task2(inputs: &Vec<Vec3>) -> i64 {
    let mut pair_distances = vec![];

    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            let dist = (inputs[j] - inputs[i]).len2();
            pair_distances.push((Vec2::new(i as i64, j as i64), dist));
        }
    }

    pair_distances.sort_by_key(|item| item.1);

    let mut circuits = FindUnion::new();
    let mut last_joined = Vec2::new(-1, -1);
    for (pair, _dist) in pair_distances {
        if circuits.join(pair.x, pair.y) {
            //println!("Joined idxs={:?} {:?} {:?}", pair, inputs[pair.x as usize], inputs[pair.y as usize]);
            last_joined = pair;
        }
    }

    return inputs[last_joined.x as usize].x * inputs[last_joined.y as usize].x;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec8() {
    let lines = read_lines("dec8.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task(&inputs, 1000);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec8_2() {
    let lines = read_lines("dec8.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task2(&inputs);
    println!("{:?}", result);
}
