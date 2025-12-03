use crate::helpers::{read_lines, read_matrix_from_lines, Matrix};

#[allow(dead_code)]
fn bank_voltage(bank: &Vec<i32>) -> i32 {
    let prefix = &bank[..bank.len() - 1];
    let best_start_value = *prefix.iter().max().unwrap();
    let best_start_index = prefix.iter().position(|&v| v == best_start_value).unwrap();
    let suffix = &bank[best_start_index + 1..];
    let best_end_value = *suffix.iter().max().unwrap();
    return best_start_value * 10 + best_end_value;
}

#[allow(dead_code)]
fn solve_task(matrix: &Matrix) -> i64 {
    let mut result: i64 = 0;
    for i in 0..matrix.height {
        let bank = matrix.get_int_row(i as i32).unwrap();
        let voltage = bank_voltage(&bank);
        result += voltage as i64;
    }
    return result;
}

#[allow(dead_code)]
fn bank_voltage2(bank: &[i32], num_batteries: usize) -> i64 {
    if num_batteries == 1 {
        return *bank.iter().max().unwrap() as i64;
    }

    let prefix = &bank[..bank.len() - num_batteries + 1];
    let best_start_value = *prefix.iter().max().unwrap();
    let best_start_index = prefix.iter().position(|&v| v == best_start_value).unwrap();
    let suffix = &bank[best_start_index + 1..];
    let best_end_value = bank_voltage2(suffix, num_batteries - 1);
    let res = (best_start_value as i64) * 10i64.pow(num_batteries as u32 - 1) + best_end_value;
    return res;
}

#[allow(dead_code)]
fn solve_task2(matrix: &Matrix) -> i64 {
    let mut result: i64 = 0;
    for i in 0..matrix.height {
        let bank = matrix.get_int_row(i as i32).unwrap();
        let voltage = crate::dec3::bank_voltage2(&bank, 12);
        result += voltage;
    }
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec3() {
    let lines = read_lines("dec3.in.txt").expect("Could not load input.");
    let matrix = read_matrix_from_lines(lines).unwrap();
    let result = solve_task(&matrix);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec3_2() {
    let lines = read_lines("dec3.in.txt").expect("Could not load input.");
    let matrix = read_matrix_from_lines(lines).unwrap();
    let result = solve_task2(&matrix);
    println!("{:?}", result);
}
