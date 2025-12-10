use std::cmp::{max, min};
use itertools::Itertools;
use regex::Regex;
use crate::basic_parsing::read_lines;
use crate::maze::Direction;
use crate::vec2::Vec2;

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Vec2>> {
    let mut inputs: Vec<Vec2> = Vec::new();
    let in_regex = Regex::new(r"^(?<x>\d+),(?<y>\d+)$").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        let cap = in_regex.captures(line).ok_or(format!("Line idx={} (zero-based) '{}' does not match the regex.", idx, line))?;

        let input = Vec2 {
            x: cap["x"].parse::<i32>().unwrap(),
            y: cap["y"].parse::<i32>().unwrap(),
        };
        inputs.push(input);
    }

    Ok(inputs)
}

fn get_area(p0: Vec2, p1: Vec2) -> i64 {
    let min_x = min(p0.x, p1.x);
    let min_y = min(p0.y, p1.y);
    let max_x = max(p0.x, p1.x);
    let max_y = max(p0.y, p1.y);
    return (max_x - min_x + 1) as i64 * (max_y - min_y + 1) as i64;
}

#[allow(dead_code)]
fn solve_task(inputs: &Vec<Vec2>) -> i64 {
    let mut pair_areas = vec![];

    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            let area = get_area(inputs[i], inputs[j]);
            pair_areas.push((Vec2::new(i as i64, j as i64), area));
        }
    }

    let result = pair_areas.iter().map(|item| item.1).max().unwrap();
    return result;
}

#[allow(dead_code)]
fn render_loop(inputs: &Vec<Vec2>) -> Vec<Vec<(i32, i32)>> {
    // Rendering spans.

    let num_lines = inputs.iter().map(|input| input.y).max().unwrap();

    fn edge_dir((p0, p1): &(Vec2, Vec2)) -> Direction {
        if p0.x == p1.x {
            if p0.y < p1.y { Direction::Down } else { Direction::Up }
        } else {
            if p0.x < p1.x { Direction::Right } else { Direction::Left }
        }
    }

    fn is_horizontal(dir: Direction) -> bool {
        (dir == Direction::Left) || (dir == Direction::Right)
    }

    let mut interior_spans_per_line = vec![vec![]; (num_lines + 1) as usize];

    for (prev_edge, cur_edge, next_edge) in inputs.iter().copied().circular_tuple_windows::<(_, _)>().circular_tuple_windows::<(_, _, _)>() {
        let (p0, p1) = cur_edge;
        let prev_dir = edge_dir(&prev_edge);
        let cur_dir = edge_dir(&cur_edge);
        let next_dir = edge_dir(&next_edge);

        if is_horizontal(cur_dir) == is_horizontal(next_dir) {
            panic!("Consecutive horiz or vert edges: cur_edge={:?}, next_edge={:?}", cur_edge, next_edge);
        }

        if is_horizontal(cur_dir) {
            let y = p0.y;
            let min_x = min(p0.x, p1.x);
            let max_x = max(p0.x, p1.x);
            if min_x == max_x {
                panic!("Horizontal edge of length 1: p0={:?}, p1={:?}", p0, p1);
            }

            if prev_dir == next_dir {
                interior_spans_per_line[y as usize].push((min_x, max_x));
            } else {
                // When we are o na bend, inject two edges.
                let avg_x = (min_x + max_x) / 2;
                interior_spans_per_line[y as usize].push((min_x, avg_x));
                interior_spans_per_line[y as usize].push((avg_x + 1, max_x));
            }
            continue;
        }

        let mut min_y = min(p0.y, p1.y);
        let mut max_y = max(p0.y, p1.y);
        if min_y == max_y {
            panic!("Vertical edge of length 1: p0={:?}, p1={:?}", p0, p1);
        }

        min_y += 1; // Skip start. Will be added by horizontal edges.
        max_y -= 1; // Skip end. Will be added by horizontal edges.
        for y in min_y..=max_y {
            interior_spans_per_line[y as usize].push((p0.x, p1.x));
        }
    }

    fn print_spans(spans: &Vec<(i32, i32)>) {
        let mut cur_pos = 0;
        for &(x0, x1) in spans {
            if x0 < cur_pos {
                panic!("x0={} < cur_pos={}", x0, cur_pos)
            }
            for _x in cur_pos..x0 {
                print!(".");
            }
            for _x in x0..=x1 {
                print!("X");
            }
            cur_pos = x1 + 1;
        }
        println!();
    }

    // Merge spans: each two consecutive spans have empty space between them filled (so they merge into one span).
    // We assume here that the loop is not self-intersecting.
    for interior_spans in interior_spans_per_line.iter_mut() {
        if interior_spans.len() == 0 {
            continue;
        }

        if (interior_spans.len() % 2) != 0 {
            panic!("Expected interior_spans to have even nuber of spans.")
        }

        interior_spans.sort();

        let merged_spans: Vec<(i32, i32)> = interior_spans.iter().tuples().map(|((x0, _x1), (_x2, x3))| (*x0, *x3)).collect_vec();

        // Now merge spans that touch.
        let mut coalesced_spans = vec![];
        let mut cur_span = *merged_spans.first().unwrap();
        for &span in merged_spans.iter().skip(1) {
            if span.0 <= cur_span.1 {
                panic!("Spans overlap: span={:?} cur_span={:?}", span, cur_span);
            }
            if span.0 == cur_span.1 + 1 {
                cur_span.1 = span.1;
                continue;
            }
            coalesced_spans.push(cur_span);
            cur_span = span;
        }
        coalesced_spans.push(cur_span);
        //print_spans(&merged_spans);

        *interior_spans = coalesced_spans;
    }

    return interior_spans_per_line;
}

#[allow(dead_code)]
fn solve_task2(inputs: &Vec<Vec2>) -> i64 {
    let spans_per_line = render_loop(inputs);
    let mut flat_spans_per_line = vec![];

    for spans in &spans_per_line {
        let mut points = vec![];
        for &(x0, x1) in spans {
            points.push(x0);
            points.push(x1);
        }
        flat_spans_per_line.push(points);
    }

    let accept_rect = |p0: Vec2, p1: Vec2| -> bool { // Correct but slow.
        let min_x = min(p0.x, p1.x);
        let min_y = min(p0.y, p1.y);
        let max_x = max(p0.x, p1.x);
        let max_y = max(p0.y, p1.y);

        fn span_match((min_x, max_x): (i32, i32), spans: &Vec::<i32>) -> bool {
            match spans.binary_search(&min_x) {
                Ok(index) => {
                    let end_pos = if (index & 1) == 0 { spans[index + 1] } else { spans[index] };
                    return max_x <= end_pos;
                },
                Err(insert_pos) => {
                    if (insert_pos & 1) == 0 { return false; }; // Span would extend before filled span.
                    let end_pos = spans[insert_pos];
                    return max_x <= end_pos;
                },
            };
        }

        for y in min_y..=max_y {
            let spans = &flat_spans_per_line[y as usize];
            if !span_match((min_x, max_x), spans) {
                return false;
            }
        }
        true
    };

    let _xaccept_rect = |p0: Vec2, p1: Vec2| -> bool { // Correct but slow.
        let min_x = min(p0.x, p1.x);
        let min_y = min(p0.y, p1.y);
        let max_x = max(p0.x, p1.x);
        let max_y = max(p0.y, p1.y);

        fn span_match((min_x, max_x): (i32, i32), spans: &Vec::<(i32, i32)>) -> bool {
            for &(span_min, span_max) in spans {
                if min_x > span_max { continue; }
                if min_x < span_min { return false; } // Tested span extends before filled span.
                if max_x > span_max { return false; } // Tested span extends after filled span.
                return true;
            }
            false
        }

        for y in min_y..=max_y {
            let spans = &spans_per_line[y as usize];
            if !span_match((min_x, max_x), spans) {
                return false;
            }
        }
        true
    };

    let mut pair_areas = vec![];

    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            let area = get_area(inputs[i], inputs[j]);
            if accept_rect(inputs[i], inputs[j]) {
                pair_areas.push((Vec2::new(i as i64, j as i64), area));
            }
        }
    }

    let result = pair_areas.iter().map(|item| item.1).max().unwrap();
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec9() {
    let lines = read_lines("dec9.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task(&inputs);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec9_2() {
    let lines = read_lines("dec9.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task2(&inputs);
    println!("{:?}", result);
}
