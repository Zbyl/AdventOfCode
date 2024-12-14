use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use crate::helpers::{read_matrix, Matrix, Vec2};

fn get_antennas(matrix: &Matrix) -> HashMap<char, HashSet<Vec2>> {
    let mut antennas = HashMap::new();

    for x in 0 .. matrix.width {
        for y in 0 .. matrix.height {
            let pos = Vec2::new(x as i32, y as i32);
            let c = matrix.get(pos).unwrap();
            if c == '.' {
                continue;
            }
            if !antennas.contains_key(&c) {
                antennas.insert(c, HashSet::new());
            }
            let s = antennas.get_mut(&c).unwrap();
            s.insert(pos);
        }
    }

    return antennas;
}

fn process_antennas(matrix: &Matrix, antennas: &HashMap<char, HashSet<Vec2>>, limit: bool) -> i32 {
    let mut results = HashSet::new();
    for (_name, positions) in antennas {
        let poss = positions.iter().collect_vec();
        for i in 0..poss.len() {
            let pos0 = *poss[i];
            for j in i+1..poss.len() {
                let pos1 = *poss[j];
                let delta = pos1 - pos0;
                let mut idx = if limit { 1 } else { 0 };
                loop {
                    let mut had = false;
                    let a1 = pos1 + delta * idx;
                    let a2 = pos0 - delta * idx;
                    if matrix.contains(a1) {
                        results.insert(a1);
                        had = true;
                    }
                    if matrix.contains(a2) {
                        results.insert(a2);
                        had = true;
                    }
                    if !had {
                        break;
                    }
                    idx += 1;
                    if limit {
                        break;
                    }
                }
            }
        }
    }
    results.len() as i32
}

#[allow(dead_code)]
pub(crate) fn dec8() {
    let matrix = read_matrix("dec8.in.txt").expect("Could not load input.");
    let antennas = get_antennas(&matrix);
    let result = process_antennas(&matrix, &antennas, true);
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec8_2() {
    let matrix = read_matrix("dec8.in.txt").expect("Could not load input.");
    let antennas = get_antennas(&matrix);
    let result = process_antennas(&matrix, &antennas, false);
    println!("{:?}", result);
}
