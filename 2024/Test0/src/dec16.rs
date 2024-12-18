use std::collections::{HashMap, HashSet};
use priority_queue::DoublePriorityQueue;
use crate::dec6::{make_maze, Direction, Maze};
use crate::helpers::{read_matrix,Matrix, Vec2};

fn turn_cost(cur_dir: Direction, next_dir: Direction) -> i64 {
    if cur_dir == next_dir { return 0; }
    if cur_dir.turn_cw() == next_dir { return 1000; }
    if cur_dir.turn_ccw() == next_dir { return 1000; }
    return 2000;
}

#[allow(dead_code)]
fn dfs(matrix: &Matrix, cur_pos: Vec2, cur_dir: Direction, visited: &mut HashSet<Vec2>, memo: &mut HashMap::<(Vec2, Direction), Option<i64>>) -> Option<i64> {
    if let Some(potential_res) = memo.get(&(cur_pos, cur_dir)) {
        return *potential_res;
    }

    if visited.contains(&cur_pos) {
        memo.insert((cur_pos, cur_dir), None);
        return None;
    }

    let cur_val = matrix.get(cur_pos).unwrap();
    if cur_val == 'E' {
        memo.insert((cur_pos, cur_dir), Some(0));
        return Some(0);
    }

    visited.insert(cur_pos);

    let mut best_result = None;

    for next_dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
        let next_pos = cur_pos + next_dir.dir();
        let val = matrix.get(next_pos).unwrap();
        if val == '#' { continue; }
        let maybe_res = dfs(matrix, next_pos, next_dir, visited, memo);
        if let Some(res) = maybe_res {
            let this_result = res + 1 + turn_cost(cur_dir, next_dir);
            if (best_result.is_none()) || (this_result < best_result.unwrap()) {
                best_result = Some(this_result);
            }
        }
    }

    visited.remove(&cur_pos);
    memo.insert((cur_pos, cur_dir), best_result);
    best_result
}

#[allow(dead_code)]
fn best_path(maze: &Maze, cur_dir: Direction) -> Option<i64> {
    let mut visited = HashSet::<Vec2>::new();
    let mut memo = HashMap::<(Vec2, Direction), Option<i64>>::new();
    return dfs(&maze.matrix, maze.start, cur_dir, &mut visited, &mut memo);
}


fn best_path2(maze: &Maze, start_dir: Direction) -> ((Vec2, Direction), HashMap::<(Vec2, Direction), i64>) {
    let mut best_dists: HashMap::<(Vec2, Direction), i64> = HashMap::new();
    let mut queue = DoublePriorityQueue::<(Vec2, Direction), i64>::new();
    queue.push((maze.start, start_dir), 0);
    loop {
        let ((cur_pos, cur_dir), cur_dist) = queue.pop_min().unwrap(); // Element with shortest distance from start.
        best_dists.insert((cur_pos, cur_dir), cur_dist);

        let cur_val = maze.matrix.get(cur_pos).unwrap();
        if cur_val == 'E' {
            return ((cur_pos, cur_dir), best_dists);
        }

        for next_dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
            let next_pos = cur_pos + next_dir.dir();
            let val = maze.matrix.get(next_pos).unwrap();
            if val == '#' { continue; }
            //queue.push_decrease((cur_pos, next_dir), cur_dist + turn_cost(cur_dir, next_dir));
            queue.push_decrease((next_pos, next_dir), cur_dist + 1 + turn_cost(cur_dir, next_dir));
        }
    }
}

fn best_path3(maze: &Maze, start_dir: Direction) -> (Vec2, HashMap::<(Vec2, Direction), i64>) {
    let mut best_dists: HashMap::<(Vec2, Direction), i64> = HashMap::new();
    let mut queue = DoublePriorityQueue::<(Vec2, Direction), i64>::new();
    queue.push((maze.start, start_dir), 0);
    let mut end_pos = None;
    let mut end_dist = None;
    loop {
        let ((cur_pos, cur_dir), cur_dist) = queue.pop_min().unwrap(); // Element with shortest distance from start.
        if end_dist.is_some() && (end_dist.unwrap() < cur_dist) {
            break;
        }
        let prev_dist = best_dists.get(&(cur_pos, cur_dir));
        if prev_dist.is_some() && (*prev_dist.unwrap() < cur_dist) {
            continue;
        }
        best_dists.insert((cur_pos, cur_dir), cur_dist);

        let cur_val = maze.matrix.get(cur_pos).unwrap();
        if cur_val == 'E' {
            end_pos = Some(cur_pos);
            end_dist = Some(cur_dist);
            continue;
        }

        for next_dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
            let (npos, ndir, ndist) =
                if next_dir == cur_dir {
                    let next_pos = cur_pos + cur_dir.dir();
                    let val = maze.matrix.get(next_pos).unwrap();
                    if val == '#' { continue; }
                    (next_pos, cur_dir, cur_dist + 1)
                } else {
                    (cur_pos, next_dir, cur_dist + turn_cost(cur_dir, next_dir))
                };

            let pdist = best_dists.get(&(npos, ndir));
            if pdist.is_none() || (*pdist.unwrap() > ndist) {
                queue.push_decrease((npos, ndir), ndist);
            }
        }
    }
    (end_pos.expect("Maze did not have end!"), best_dists)
}

fn count_best_seats(end_pos: Vec2, best_dists: &HashMap::<(Vec2, Direction), i64>) -> i64 {
    let mut best_seats = HashSet::<Vec2>::new();
    let mut wavefront = Vec::<(Vec2, Direction, i64)>::new();

    let mut end_dist = None;
    for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
        let dist = best_dists.get(&(end_pos, dir));
        if dist.is_none() { continue; }
        if end_dist.is_none() { end_dist = dist; continue; }
        if (dist < end_dist) { end_dist = dist; continue; }
    }

    for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
        let dist = best_dists.get(&(end_pos, dir));
        if dist.is_none() { continue; }
        if (dist != end_dist) { continue; }
        wavefront.push((end_pos, dir, *dist.unwrap()));
        best_seats.insert(end_pos);
    }

    //println!("{:?}", best_dists.get(&(Vec2::new(12, 13), Direction::Right)));

    loop {
        //println!("============================================");
        if wavefront.is_empty() { break; }

        let mut new_wavefront = Vec::<(Vec2, Direction, i64)>::new();
        for (cur_pos, cur_dir, cur_dist) in wavefront.iter() {
            //println!("{:?} {:?} {:?}", cur_pos, cur_dir, cur_dist);
            for next_dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let next_pos = *cur_pos + next_dir.turn_cw().turn_cw().dir();
                let next_dist = best_dists.get(&(next_pos, next_dir));
                if next_dist.is_none() { continue; }

                let step_cost = if next_dir == *cur_dir { 1 } else { turn_cost(*cur_dir, next_dir) + 1 };
                if *next_dist.unwrap() + step_cost == *cur_dist {
                    new_wavefront.push((next_pos, next_dir, *next_dist.unwrap()));
                    best_seats.insert(next_pos);
                }
            }
        }

        wavefront = new_wavefront;
    }

    best_seats.len() as i64
}

#[allow(dead_code)]
pub(crate) fn dec16() {
    let matrix = read_matrix("dec16.in.txt").expect("Could not load input.");
    let maze = make_maze(matrix, 'S');
    let ((end_pos, end_dir), best_dists) = best_path2(&maze, Direction::Right);
    let best_dist = best_dists[&(end_pos, end_dir)];
    println!("{:?}", best_dist);
}

#[allow(dead_code)]
pub(crate) fn dec16_2() {
    let matrix = read_matrix("dec16.in.txt").expect("Could not load input.");
    let maze = make_maze(matrix, 'S');
    let (end_pos, best_dists) = best_path3(&maze, Direction::Right);
    for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
        let dist = best_dists.get(&(end_pos, dir));
        println!("{:?} {:?}", dir, dist);
    }
    let best_seat_count = count_best_seats(end_pos, &best_dists);
    println!("{:?}", best_seat_count);
}
