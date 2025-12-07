use std::collections::HashSet;
use crate::basic_parsing::read_lines;
use crate::matrix::{read_matrix_from_lines, Matrix};
use crate::vec2::Vec2;

#[allow(dead_code)]
fn solve_task(matrix: &Matrix) -> i64 {
    let mut start_pos = Vec2::new(0, 0);
    let mut chevrons = vec![hashset![]; matrix.height];
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos = Vec2::new(x as i32, y as i32);
            let c = matrix.get(pos).unwrap();
            if c == 'S' {
                start_pos = pos;
            } else if c == '^' {
                chevrons[y].insert(x as i32);
            }
        }
    }

    let mut beams = HashSet::from([start_pos.x]); // Beam positions.

    let mut num_splits = 0;
    for (_line_idx, chevron_line) in chevrons.iter().enumerate() {
        let mut new_beams = HashSet::from([]); // Beam positions.
        for beam in beams {
            if chevron_line.contains(&beam) {
                num_splits += 1;
                if beam > 0 {
                    new_beams.insert(beam - 1);
                }
                if beam < (matrix.width as i32) - 1 {
                    new_beams.insert(beam + 1);
                }
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }
    return num_splits;
}
/*

.......S.......
.......1.......
......1^1......
......1.1......
.....1^2^1.....
.....1.2.1.....
....1^3^3^1....
....1.3.3.1....
...1^4^331^1...
...1.4.331.1...
..1^5^434^2^1..
..1.5.434.2.1..
.1^154^74.21^1.
.1.154.74.21.1.
1^2^D^J^J^211^1
1.2.D.J.J.211.1

 */

#[allow(dead_code)]
fn solve_task2(matrix: &Matrix) -> i64 {
    let mut start_pos = Vec2::new(0, 0);
    let mut chevrons = vec![hashset![]; matrix.height];
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            let pos = Vec2::new(x as i32, y as i32);
            let c = matrix.get(pos).unwrap();
            if c == 'S' {
                start_pos = pos;
            } else if c == '^' {
                chevrons[y].insert(x as i32);
            }
        }
    }

    let mut beams = HashSet::from([start_pos.x]); // Beam positions.
    let mut num_incoming_timelines = vec![0; matrix.width];
    num_incoming_timelines[start_pos.x as usize] = 1;

    let mut _num_splits = 0;
    for (_line_idx, chevron_line) in chevrons.iter().enumerate() {
        let mut new_beams = HashSet::from([]); // Beam positions.
        let mut new_incoming_timelines = vec![0; matrix.width];
        for beam in beams {
            if chevron_line.contains(&beam) {
                _num_splits += 1;
                if beam > 0 {
                    new_beams.insert(beam - 1);
                    new_incoming_timelines[(beam - 1) as usize] += num_incoming_timelines[beam as usize];
                }
                if beam < (matrix.width as i32) - 1 {
                    new_beams.insert(beam + 1);
                    new_incoming_timelines[(beam + 1) as usize] += num_incoming_timelines[beam as usize];
                }
            } else {
                new_beams.insert(beam);
                new_incoming_timelines[beam as usize] += num_incoming_timelines[beam as usize];
            }
        }
        beams = new_beams;
        num_incoming_timelines = new_incoming_timelines;

        fn print_timeline(timeline: &Vec<i64>) {
            for count in timeline {
                print!("{:2} ", count);
            }
            println!("");
        }

        //print_timeline(&num_incoming_timelines);
    }
    let result = num_incoming_timelines.iter().sum();
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec7() {
    let lines = read_lines("dec7.in.txt").expect("Could not load input.");
    let mut matrix = read_matrix_from_lines(lines).unwrap();
    let result = solve_task(&mut matrix);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec7_2() {
    let lines = read_lines("dec7.in.txt").expect("Could not load input.");
    let mut matrix = read_matrix_from_lines(lines).unwrap();
    let result = solve_task2(&mut matrix);
    println!("{:?}", result);
}
