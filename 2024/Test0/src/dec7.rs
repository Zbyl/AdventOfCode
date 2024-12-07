use itertools::Itertools;
use regex::Regex;
use crate::helpers::read_lines;

#[derive(Debug)]
pub(crate) struct Task {
    result: i64,
    numbers: Vec<i64>,
}

pub(crate) fn read_dec7_input(filename: &str) -> crate::helpers::Result<Vec<Task>> {
    let lines = read_lines(filename)?;

    let mut tasks: Vec<Task> = Vec::new();

    let rule_regex = Regex::new(r"^(?<result>\d+): (?<numbers>\d+(?: \d+)*)$").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let Some(caps) = rule_regex.captures(line) else {
            return Err(format!("Line idx={} (zero-based) {} does not match the rule regex.", idx, line).into());
        };
        let task = Task {
            result: caps["result"].parse::<i64>().unwrap(),
            numbers: caps["numbers"].split(" ").map(|c| c.parse::<i64>().unwrap()).collect_vec(),
        };
        tasks.push(task);
    }

    Ok(tasks)
}

fn solve_task(task: &Task) -> bool {
    let num = task.numbers.len() as i32;
    if num == 0 {
        return task.result == 0;
    }
    if num > 60 {
        panic!("Too many numbers: {:?}", task.numbers);
    }

    let all_combs: i64 = 1i64 << (num - 1);
    for n in 0..all_combs {
        let mut res = task.numbers[0];
        for i in 1..num {
            let bit = (n & (1i64 << (i - 1))) != 0;
            if !bit {
                res += task.numbers[i as usize];
            } else {
                res *= task.numbers[i as usize];
            }
        }
        if res == task.result {
            return true;
        }
    }
    return false;
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Op {
    Add,
    Mul,
    Con,
}

fn solve_task2(task: &Task) -> bool {
    let num = task.numbers.len() as i32;
    if num == 0 {
        return task.result == 0;
    }

    let mut comb = vec![Op::Add; (num - 1) as usize];

    fn inc(comb: &mut Vec<Op>)-> bool {
        let mut inc_pos = 0;
        loop {
            match comb[inc_pos] {
                Op::Add => { comb[inc_pos] = Op::Mul; return true; }
                Op::Mul => { comb[inc_pos] = Op::Con; return true;  }
                Op::Con => {
                    comb[inc_pos] = Op::Add;
                    inc_pos += 1;
                    if inc_pos >= comb.len() {
                        return false;
                    }
                }
            }
        }
    }

    loop {
        let mut res = task.numbers[0];
        for (idx, op) in comb.iter().cloned().enumerate() {
            match op {
                Op::Add => { res += task.numbers[idx + 1]; }
                Op::Mul => { res *= task.numbers[idx + 1]; }
                Op::Con => {
                    let n = task.numbers[idx + 1];
                    let x = n.ilog10();
                    res = res * 10i64.pow(x + 1) + n;
                }
            }
        }
        //println!("{} {} {:?}", task.result, res, comb);
        if res == task.result {
            return true;
        }

        if !inc(&mut comb) {
            return false;
        }
    }
}

fn dec7_count(tasks: &Vec<Task>, sec: bool) -> i64 {
    let mut result = 0;
    for task in tasks {
        if (if sec { solve_task2(task) } else {solve_task(task)}) {
            result += task.result;
        }
    }
    return result;
}

#[allow(dead_code)]
pub(crate) fn dec7() {
    let tasks = read_dec7_input("dec7.in.txt").expect("Error loading input.");
    let result = dec7_count(&tasks, false);
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec7_2() {
    let tasks = read_dec7_input("dec7.in.txt").expect("Error loading input.");
    let result = dec7_count(&tasks, true);
    println!("{:?}", result);
}
