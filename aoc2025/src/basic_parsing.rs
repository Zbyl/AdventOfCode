use std::fs::read_to_string;
use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;
use itertools::Itertools;
use crate::helpers::Result;

#[allow(dead_code)]
pub(crate) fn read_lines(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let contents = read_to_string(filename)?;

    for line in contents.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

#[allow(dead_code)]
pub(crate) fn read_line(filename: &str) -> Result<String> {
    let result = read_lines(filename)?;
    if result.is_empty() {
        return Err(From::from(format!("No lines in file: {}", filename)));
    }
    if result.len() > 1 {
        return Err(From::from(format!("Expected only one line in file {}, but got: {}", filename, result.len())));
    }

    Ok(result.first().unwrap().clone())
}

#[allow(dead_code)]
pub(crate) fn parse_nums<T: FromStr>(content: &str) -> Vec<T>
    where <T as FromStr>::Err: Debug
{
    let pieces = content.split(' ')
        .map(|s| s.parse::<T>().expect(format!("Cannot parse {} as {}.", s, type_name::<T>()).as_str()))
        .collect_vec();
    pieces
}

#[allow(dead_code)]
/// Separates input lines into two lists. Break is on first blank line.
pub fn separate_by_blank(lines: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut s0: Vec<String> = Vec::new();
    let mut s1: Vec<String> = Vec::new();
    let mut first = true;
    for line in lines {
        if line.is_empty() {
            first = false;
            continue;
        }
        if first {
            s0.push(line.clone());
        } else {
            s1.push(line.clone());
        }
    }

    (s0, s1)
}
