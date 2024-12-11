use std::collections::HashMap;
use crate::helpers::{parse_nums, read_line};

fn compute_step(nums: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();
    for &num in nums {
        if num == 0 {
            result.push(1);
            continue;
        }
        let num_digits = if num < 10 { 1 } else { num.ilog10() + 1 };
        if num_digits & 1 == 0 {
            let p = 10i64.pow(num_digits / 2);
            result.push(num / p);
            result.push(num % p);
            continue;
        }
        result.push(num * 2024);
    }
    result
}

fn compute_step_single(num: i64) -> Vec<i64> {
    let nums = vec![num];
    compute_step(&nums)
}

fn compute_result(nums: Vec<i64>, steps: i64) -> Vec<i64> {
    let mut result = nums;
    //println!("{:?}", result);
    for step in 0..steps {
        result = compute_step(&result);
        println!("{}: {}", step, result.len());
    }
    result
}

fn memoize_step(num: i64, steps: i32, cache: &mut HashMap<(i64, i32), i64>) -> i64 {
    if steps == 0 { return 1 }

    if let Some(&rock_count) = cache.get(&(num, steps)) {
        return rock_count;
    }

    let rocks = compute_step_single(num);
    let mut rock_count = 0;
    for rock in rocks {
        rock_count += memoize_step(rock, steps - 1, cache);
    }

    cache.insert((num, steps), rock_count);

    rock_count
}

fn compute_result2(nums: Vec<i64>, steps: i32) -> i64 {
    let mut cache: HashMap<(i64, i32), i64> = HashMap::new();
    let mut rock_count = 0;
    for num in nums {
        rock_count += memoize_step(num, steps, &mut cache);
    }
    rock_count
}


#[allow(dead_code)]
pub(crate) fn dec11() {
    let content = read_line("dec11.in.txt").expect("Could not load input.");
    let nums = parse_nums(&content);
    let result = compute_result(nums, 25);
    println!("{:?}", result.len());
}

#[allow(dead_code)]
pub(crate) fn dec11_2() {
    let content = read_line("dec11.in.txt").expect("Could not load input.");
    let nums = parse_nums(&content);
    let result = compute_result2(nums, 75);
    println!("{:?}", result);
}
