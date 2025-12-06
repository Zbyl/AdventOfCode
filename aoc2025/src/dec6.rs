use itertools::Itertools;
use regex::Regex;
use crate::basic_parsing::read_lines;


#[derive(Debug, Clone)]
struct Input{
    nums: Vec<i64>,
    operator: char,
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Input>> {
    let in_regex = Regex::new(r"\s+").unwrap();

    let operator_line = lines.last().unwrap();
    let ops = in_regex.split(operator_line.trim()).map(|s| s.chars().next().unwrap()).collect_vec();

    let number_lines = &lines[0..lines.len() - 1];
    let mut num_lines: Vec<Vec<i64>> = vec![];
    for (_idx, line) in number_lines.iter().enumerate() {
        let nums = in_regex.split(line.trim()).map(|s| s.parse::<i64>().unwrap()).collect_vec();
        num_lines.push(nums);
    }

    let mut inputs: Vec<Input> = Vec::new();
    for i in 0..ops.len() {
        let input = Input {
            nums: num_lines.iter().map(|v| v[i]).collect_vec(),
            operator: ops[i],
        };
        inputs.push(input);
    }

    Ok(inputs)
}

fn parse_input2(input_lines: &Vec<String>) -> crate::helpers::Result<Vec<Input>> {
    let in_regex = Regex::new(r"\s+").unwrap();
    let operator_line = input_lines.last().unwrap();
    let ops = in_regex.split(operator_line.trim()).map(|s| s.chars().next().unwrap()).collect_vec();

    let number_lines = &input_lines[0..input_lines.len() - 1];
    let longest_line = number_lines.iter().map(|l| l.len()).max().unwrap();
    let mut columns = vec![vec![]; longest_line];

    for (_line_idx, line) in number_lines.iter().enumerate() {
        for (char_idx, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            columns[char_idx].push(c);
        }
    }


    let mut inputs: Vec<Input> = Vec::new();
    let mut cur_column = 0;
    for op in ops {
        let mut nums = vec![];
        loop {
            if cur_column >= longest_line {
                break;
            }
            let col = &columns[cur_column];
            if col.len() == 0 {
                cur_column += 1;
                break;
            }

            nums.push(col_to_value(col));

            cur_column += 1;
        }
        let input = Input {
            nums: nums,
            operator: op,
        };
        inputs.push(input);
    }

    Ok(inputs)
}

fn col_to_value(s: &Vec<char>) -> i64 {
    let mut result = 0;
    for c in s {
        result = result * 10 + c.to_digit(10).unwrap() as i64;
    }
    return result;
}

fn compute_stuff(inputs: &Vec<Input>) -> i64 {
    let mut result = 0;
    for input in inputs {
        if input.operator == '+' {
            result += input.nums.iter().sum::<i64>();
        } else {
            result += input.nums.iter().product::<i64>();
        }
    }
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec6() {
    let lines = read_lines("dec6.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = compute_stuff(&inputs);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec6_2() {
    let lines = read_lines("dec6.in.txt").expect("Could not load input.");
    let inputs = parse_input2(&lines).unwrap();
    println!("Inputs: {:?}", inputs);
    let result = compute_stuff(&inputs);
    println!("{:?}", result);
}
