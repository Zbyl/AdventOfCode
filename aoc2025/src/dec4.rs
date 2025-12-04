use std::collections::HashSet;
use crate::basic_parsing::read_lines;
use crate::matrix::{find_points, read_matrix_from_lines, Matrix};
use crate::vec2::Vec2;

#[allow(dead_code)]
fn solve_task(matrix: &Matrix) -> i64 {
    let mut accessible_rolls: i64 = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos = Vec2::new(x as i32, y as i32);

            if matrix.get(pos).unwrap() != '@' {
                continue;
            }

            let mut neighbour_rolls = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 { continue; }
                    let dpos = pos + Vec2::new(dx, dy);
                    if matrix.get(dpos).unwrap_or('.') == '@' {
                        neighbour_rolls += 1;
                    }
                }
            }

            if neighbour_rolls < 4 {
                accessible_rolls += 1;
            }
        }
    }
    return accessible_rolls;
}

#[allow(dead_code)]
fn solve_task2(mut matrix: &mut Matrix) -> i64 {
    let points = find_points(&mut matrix, None, &HashSet::from(['@']));
    if points.is_empty() {
        return 0;
    }
    let mut maybe_awake_rolls: HashSet<Vec2> = points.get(&'@').unwrap().into_iter().copied().collect();
    let mut removed_rolls_count = 0;

    loop {
        if maybe_awake_rolls.len() == 0 {
            break;
        }

        let mut new_awake_rolls: HashSet<Vec2> = HashSet::new();

        'next_roll:
        for &roll in &maybe_awake_rolls {
            // Check if roll was not removed before
            if matrix.get(roll).unwrap_or('.') != '@' {
                continue 'next_roll;
            }

            // Check if roll can be accessed by a forklift.
            let mut neighbour_rolls = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 { continue; }
                    let dpos = roll + Vec2::new(dx, dy);
                    if matrix.get(dpos).unwrap_or('.') == '@' {
                        neighbour_rolls += 1;
                        if neighbour_rolls >=4 {
                            // Current roll is not awaken. Skip to the next one.
                            continue 'next_roll;
                        }
                    }
                }
            }

            // Removing current roll, but waking up neighboring rolls.
            new_awake_rolls.insert(roll);
            matrix.put(roll, '.');
            removed_rolls_count += 1;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 { continue; }
                    let dpos = roll + Vec2::new(dx, dy);
                    if matrix.get(dpos).unwrap_or('.') == '@' {
                        new_awake_rolls.insert(dpos);
                    }
                }
            }
        }

        maybe_awake_rolls = new_awake_rolls;
    }

    return removed_rolls_count;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec4() {
    let lines = read_lines("dec4.in.txt").expect("Could not load input.");
    let matrix = read_matrix_from_lines(lines).unwrap();
    let result = solve_task(&matrix);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec4_2() {
    let lines = read_lines("dec4.in.txt").expect("Could not load input.");
    let mut matrix = read_matrix_from_lines(lines).unwrap();
    let result = solve_task2(&mut matrix);
    println!("{:?}", result);
}
