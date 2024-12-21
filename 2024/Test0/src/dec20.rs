use std::collections::{HashMap, HashSet};
use crate::dec6::{make_maze, Direction, Maze};
use crate::helpers::{find_single_points, print_matrix, read_lines, read_matrix, Matrix, Vec2};

fn shortest_path(matrix: &Matrix, start_pos: Vec2, end_pos: Vec2) -> i64 {
    let mut best_distances : HashMap<Vec2, i64> = HashMap::new();
    let mut wavefront: Vec<Vec2> = Vec::new();
    wavefront.push(start_pos);
    best_distances.insert(start_pos, 0);
    loop {
        if wavefront.is_empty() {
            panic!("No path to end.");
        }

        let mut new_wavefront = Vec::new();
        for pos in wavefront {
            let dist = *best_distances.get(&pos).unwrap();
            for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let next_pos = pos + dir.dir();
                let val = matrix.get(next_pos);
                if val.is_none() { continue; }
                if val == Some('#') { continue; }
                if next_pos == end_pos {
                    return dist + 1;
                }
                if best_distances.contains_key(&next_pos) { continue; }
                new_wavefront.push(next_pos);
                best_distances.insert(next_pos, dist + 1);
            }
        }
        wavefront = new_wavefront;
    }
}

fn shortest_paths_to_walls(matrix: &Matrix, start_pos: Vec2) -> HashMap<Vec2, i64> {
    let mut best_distances : HashMap<Vec2, i64> = HashMap::new();
    let mut best_wall_distances : HashMap<Vec2, i64> = HashMap::new();
    let mut wavefront: Vec<Vec2> = Vec::new();
    wavefront.push(start_pos);
    best_distances.insert(start_pos, 0);
    loop {
        if wavefront.is_empty() {
            break;
        }

        let mut new_wavefront = Vec::new();
        for pos in wavefront {
            let dist = *best_distances.get(&pos).unwrap();
            for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let next_pos = pos + dir.dir();
                let val = matrix.get(next_pos);
                if val.is_none() { continue; }
                if val == Some('#') {
                    if !best_wall_distances.contains_key(&next_pos) {
                        best_wall_distances.insert(next_pos, dist + 1);
                    }
                    continue;
                }
                if best_distances.contains_key(&next_pos) { continue; }
                new_wavefront.push(next_pos);
                best_distances.insert(next_pos, dist + 1);
            }
        }
        wavefront = new_wavefront;
    }
    best_wall_distances
}

fn shortest_paths_to_walls2(matrix: &Matrix, start_pos: Vec2) -> HashMap<Vec2, i64> {
    let mut best_distances : HashMap<Vec2, i64> = HashMap::new();
    let mut best_wall_distances : HashMap<Vec2, i64> = HashMap::new();
    let mut wavefront: Vec<Vec2> = Vec::new();
    wavefront.push(start_pos);
    best_distances.insert(start_pos, 0);
    loop {
        if wavefront.is_empty() {
            break;
        }

        let mut new_wavefront = Vec::new();
        for pos in wavefront {
            let dist = *best_distances.get(&pos).unwrap();
            for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let next_pos = pos + dir.dir();
                let val = matrix.get(next_pos);
                if val.is_none() { continue; }
                if val == Some('#') {
                    if !best_wall_distances.contains_key(&pos) {
                        best_wall_distances.insert(pos, dist);
                    }
                    continue;
                }
                if best_distances.contains_key(&next_pos) { continue; }
                new_wavefront.push(next_pos);
                best_distances.insert(next_pos, dist + 1);
            }
        }
        wavefront = new_wavefront;
    }
    best_wall_distances
}

fn shortest_path_cheats(matrix: &Matrix, start: Vec2, max_cheats: i32) -> Vec<(Option<i32>, i64)> {
    let mut best_distances : HashMap<Vec2, (Option<i32>, i64)> = HashMap::new(); // pos -> (cheats, dist from start)
    let mut visited : HashSet<(Option<i32>, Vec2)> = HashSet::new(); // (cheats, pos)
    let mut wavefront: Vec<Vec2> = Vec::new();
    wavefront.push(start);
    visited.insert((None, start));
    best_distances.insert(start, (None, 0));
    let mut ends: Vec<(Option<i32>, i64)> = Vec::new();
    loop {
        if wavefront.is_empty() {
            break;
        }

        let mut new_wavefront = Vec::new();
        for pos in wavefront {
            let (cheats, dist) = *best_distances.get(&pos).unwrap();
            for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let next_pos = pos + dir.dir();
                let val = matrix.get(next_pos);
                if val.is_none() { continue; }
                let mut next_cheats;
                if val == Some('#') {
                    match cheats {
                        None => next_cheats = Some(1),
                        Some(v@max_cheats) => { continue; },
                        Some(v) => { next_cheats = Some(v + 1); }
                    }
                } else {
                    match cheats {
                        None => next_cheats = None,
                        Some(v@max_cheats) => next_cheats = cheats,
                        Some(v) => { next_cheats = Some(v + 1); }
                    }
                }
                if val == Some('E') {
                    ends.push((next_cheats, dist + 1));
                    continue;
                }
                if visited.contains(&(next_cheats, next_pos)) { continue; }
                new_wavefront.push(next_pos);
                best_distances.insert(next_pos, (next_cheats, dist + 1));
            }
        }
        wavefront = new_wavefront;
    }

    ends
}

fn find_best_shortcuts(swalls: &HashMap<Vec2, i64>, ewalls: &HashMap<Vec2, i64>, max_len: i64) -> Vec<(Vec2, i64)> {
    let mut result: Vec<(Vec2, i64)> = Vec::new();
    for (spos, sdist) in swalls {
        if !ewalls.contains_key(spos) { continue; }
        let &edist = ewalls.get(spos).unwrap();
        let dist = sdist + edist;
        if dist > max_len { continue; }
        result.push((*spos, dist));
    }
    result
}

fn find_best_shortcuts2(swalls: &HashMap<Vec2, i64>, ewalls: &HashMap<Vec2, i64>, cheat_dist: i64, max_len: i64) -> Vec<(Vec2, i64)> {
    let mut result: Vec<(Vec2, i64)> = Vec::new();
    for (spos, sdist) in swalls {
        for (epos, edist) in ewalls.iter() {
            let mdist = ((epos.x - spos.x).abs() + (epos.y - spos.y).abs()) as i64;
            if mdist > cheat_dist { continue; }
            let dist = sdist + mdist + edist;
            if dist > max_len { continue; }
            result.push((*spos, dist));
        }
    }
    result
}

#[allow(dead_code)]
pub(crate) fn dec20() {
    let mut matrix = read_matrix("dec20.in.txt").expect("Could not load input.");
    //let mut maze = make_maze(matrix, 'S');
    let points = find_single_points(&mut matrix, Some('.'), &HashSet::from(['S', 'E']), true);
    let start_pos = points[&'S'];
    let end_pos = points[&'E'];
    print_matrix(&matrix, &hashmap! {});
    let spath = shortest_path(&matrix, start_pos, end_pos);
    //let ends = shortest_path_cheats(&matrix, start_pos, end_pos, 1);
    let swalls = shortest_paths_to_walls(&matrix, start_pos);
    let ewalls = shortest_paths_to_walls(&matrix, end_pos);
    let best_shortcuts = find_best_shortcuts(&swalls, &ewalls, spath - 100);
    println!("{:?}", spath);
    println!("{:?}", best_shortcuts.len());
    //println!("{:?}", swalls);
    //println!("{:?}", ewalls);
}

#[allow(dead_code)]
pub(crate) fn dec20_2() {
    let mut matrix = read_matrix("dec20.in.txt").expect("Could not load input.");
    //let mut maze = make_maze(matrix, 'S');
    let points = find_single_points(&mut matrix, Some('.'), &HashSet::from(['S', 'E']), true);
    let start_pos = points[&'S'];
    let end_pos = points[&'E'];
    print_matrix(&matrix, &hashmap! {});
    let spath = shortest_path(&matrix, start_pos, end_pos);
    //let ends = shortest_path_cheats(&matrix, start_pos, end_pos, 1);
    let swalls = shortest_paths_to_walls2(&matrix, start_pos);
    let ewalls = shortest_paths_to_walls2(&matrix, end_pos);
    let best_shortcuts = find_best_shortcuts2(&swalls, &ewalls, 20, spath - 100);
    println!("{:?}", spath);
    println!("{:?}", best_shortcuts.len());
    //println!("{:?}", swalls);
    //println!("{:?}", ewalls);
}
