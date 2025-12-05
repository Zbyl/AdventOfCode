use std::cmp::max;
use itertools::Itertools;
use regex::Regex;
use crate::basic_parsing::{read_lines, separate_by_blank};

#[derive(Debug, Clone, Copy)]
struct InclusiveRange {
    start: i64, // Inclusive.
    end: i64,   // Inclusive.
}

#[derive(Debug, Clone, Copy)]
struct ExclusiveRange {
    start: i64, // Inclusive.
    end: i64,   // Exclusive.
}

#[allow(dead_code)]
impl InclusiveRange {
    pub fn contains(&self, id: i64) -> bool {
        (self.start <= id) && (id <= self.end)
    }
}

#[allow(dead_code)]
impl ExclusiveRange {
    pub fn contains(&self, id: i64) -> bool {
        (self.start <= id) && (id < self.end)
    }
}

impl From<InclusiveRange> for ExclusiveRange {
    fn from(range: InclusiveRange) -> Self {
        ExclusiveRange { start: range.start, end: range.end + 1 }
    }
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<InclusiveRange>> {
    let mut inputs: Vec<InclusiveRange> = Vec::new();

    let in_regex = Regex::new(r"^(?<start>\d+)-(?<end>\d+)$").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        let cap = in_regex.captures(line).ok_or(format!("Line idx={} (zero-based) '{}' does not match the regex.", idx, line))?;

        let input = InclusiveRange {
            start: cap["start"].parse::<i64>().unwrap(),
            end: cap["end"].parse::<i64>().unwrap(),
        };
        inputs.push(input);
    }

    Ok(inputs)
}

#[allow(dead_code)]
fn solve_task(ranges: &Vec<InclusiveRange>, ids: &Vec<i64>) -> i64 {
    let mut result = 0;
    for &id in ids {
        for range in ranges {
            if range.contains(id) {
                result += 1;
                break;
            }
        }
    }
    return result;
}

#[allow(dead_code)]
fn solve_task2(ranges: &Vec<ExclusiveRange>) -> i64 {
    if ranges.len() == 0 {
        return 0;
    }

    let sorted_ranges: Vec<ExclusiveRange> = ranges.iter().sorted_by_key(|r| (r.start, r.end)).copied().collect();

    let mut result = 0;
    let mut cur_pos = sorted_ranges.first().unwrap().start;
    let mut cur_idx = 0;
    loop {
        if cur_idx >= ranges.len() {
            break
        }
        let range = sorted_ranges[cur_idx];
        cur_pos = max(cur_pos, range.start);
        if range.end > cur_pos {
            result += range.end - cur_pos;
            cur_pos = range.end;
        }
        cur_idx += 1;
    }
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec5() {
    let lines = read_lines("dec5.in.txt").expect("Could not load input.");
    let (range_lines, id_lines) = separate_by_blank(&lines);
    let ranges = parse_input(&range_lines).unwrap();
    let ids = id_lines.iter().map(|id| id.parse::<i64>().unwrap()).collect();
    let result = solve_task(&ranges, &ids);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec5_2() {
    let lines = read_lines("dec5.in.txt").expect("Could not load input.");
    let (range_lines, id_lines) = separate_by_blank(&lines);
    let in_ranges = parse_input(&range_lines).unwrap();
    let ex_ranges = in_ranges.into_iter().map(Into::into).collect();
    let result = solve_task2(&ex_ranges);
    println!("{:?}", result);
}
