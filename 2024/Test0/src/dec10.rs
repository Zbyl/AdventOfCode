use std::collections::HashSet;
use crate::helpers::{read_matrix, Matrix, Vec2};

fn trail_score(matrix: &Matrix, start_pos: Vec2) -> i32 {
    let mut visited: HashSet<Vec2> = HashSet::new();
    let mut path: Vec<Vec2> = Vec::new();
    let mut score = 0;

    let dirs = vec![ Vec2::new(0, -1), Vec2::new(1, 0), Vec2::new(0, 1), Vec2::new(-1, 0), ];

    path.push(start_pos);

    loop {
        if path.is_empty() {
            break;
        }
        let cur_pos = path.pop().unwrap();
        if visited.contains(&cur_pos) {
            continue;
        }
        visited.insert(cur_pos);

        let cur_height = matrix.get_int(cur_pos).unwrap();
        if cur_height == 9 {
            score += 1;
            continue;
        }

        for dir in dirs.iter() {
            let next_pos = cur_pos + *dir;
            if !matrix.contains(next_pos) {
                continue;
            }

            let next_height = matrix.get_int(next_pos).unwrap();
            if next_height != cur_height + 1 {
                continue;
            }
            path.push(next_pos);
        }
    }

    score
}

fn compute_result(matrix: &Matrix) -> i32 {
    let mut result = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos= Vec2::new(x as i32, y as i32);
            let val = matrix.get_int(pos).unwrap();
            if val != 0 {
                continue;
            }
            result += trail_score(matrix, pos);
        }
    }
    result
}

#[allow(dead_code)]
pub(crate) fn dec10() {
    let matrix = read_matrix("dec10.in.txt").expect("Could not load input.");
    let result = compute_result(&matrix);
    println!("{:?}", result);
}
