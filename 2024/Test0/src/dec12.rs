use std::collections::HashSet;
use crate::find_union::FindUnion;
use crate::helpers::{read_matrix, Matrix, Vec2};

static DIRS: [Vec2; 4] = [Vec2::new(0, -1), Vec2::new(0, 1), Vec2::new(-1, 0), Vec2::new(1, 0)];

fn flood(matrix: &Matrix, start_pos: Vec2, mid: char, visited: &mut HashSet<Vec2>) -> (i32, i32) {
    if visited.contains(&start_pos) {
        return (0, 0);
    }

    visited.insert(start_pos);

    let mut peri = 4;
    let mut arr = 1;
    for &dir in &DIRS {
        let new_pos = start_pos + dir;
        let c = matrix.get(new_pos).unwrap_or('.');
        if c == mid {
            let (addper, addarr) = flood(&matrix, new_pos, mid, visited);
            peri += addper - 1;
            arr += addarr;
        }
    }

    (peri, arr)
}

fn compute_result(matrix: &Matrix) -> i64 {
    let mut visited: HashSet<Vec2> = HashSet::new();
    let mut result = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos = Vec2::new(x as i32, y as i32);
            let (peri, arr) = flood(matrix, pos, matrix.get(pos).unwrap(), &mut visited);
            result += (peri as i64) * (arr as i64);
        }
    }
    result
}

fn flood2(matrix: &Matrix, start_pos: Vec2, mid: char, visited: &mut HashSet<Vec2>, fences: &mut FindUnion<(Vec2, Vec2)>) -> (i32, i32) {
    if visited.contains(&start_pos) {
        return (0, 0);
    }

    visited.insert(start_pos);

    let mut peri = 4;
    let mut arr = 1;
    for &dir in &DIRS {
        let new_pos = start_pos + dir;
        let c = matrix.get(new_pos).unwrap_or('.');
        if c == mid {
            let (addper, addarr) = flood2(&matrix, new_pos, mid, visited, fences);
            peri += addper - 1;
            arr += addarr;
        } else {
            fences.ensure((start_pos, dir));
            if fences.contains((start_pos + dir.rot_cw(), dir)) {
                fences.join((start_pos + dir.rot_cw(), dir), (start_pos, dir));
            }
            if fences.contains((start_pos + dir.rot_ccw(), dir)) {
                fences.join((start_pos + dir.rot_ccw(), dir), (start_pos, dir));
            }
        }
    }

    (peri, arr)
}

fn compute_result2(matrix: &Matrix) -> i64 {
    let mut visited: HashSet<Vec2> = HashSet::new();
    let mut result = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let mut fences: FindUnion<(Vec2, Vec2)> = FindUnion::new();
            let pos = Vec2::new(x as i32, y as i32);
            let (_peri, arr) = flood2(matrix, pos, matrix.get(pos).unwrap(), &mut visited, &mut fences);
            let sides = fences.get_sets();
            let num_sides = sides.len();
            result += (num_sides as i64) * (arr as i64);
        }
    }
    result
}

#[allow(dead_code)]
pub(crate) fn dec12() {
    let matrix = read_matrix("dec12.in.txt").expect("Could not load input.");
    let result = compute_result(&matrix);
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec12_2() {
    let matrix = read_matrix("dec12.in.txt").expect("Could not load input.");
    let result = compute_result2(&matrix);
    println!("{:?}", result);
}
