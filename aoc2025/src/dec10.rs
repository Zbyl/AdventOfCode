use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use nom::character::char;
use nom::combinator::all_consuming;
use nom::sequence::delimited;
use crate::basic_parsing::read_lines;
use nom::{IResult, Parser};
use nom::{bytes::complete::tag, multi::separated_list1};
use nom::branch::alt;
use nom::character::complete::i64 as cc_i64;
use nom::multi::many1;

#[derive(Debug, Clone)]
struct Input {
    leds: Vec::<bool>,
    buttons: Vec<Vec<i64>>,
    joltages: Vec<i64>,
}

/// Parses 1,10,53
fn parse_number_list(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(","), cc_i64).parse(input)
}

/// Parses (3) (1,3) (2) (2,3)
fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(tag(" "), delimited(tag("("), parse_number_list, tag(")"))).parse(input)
}

/// Parses [..##..]
fn parse_leds(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(char('['), many1(alt((tag("."), tag("#")))), char(']')).parse(input)
        .map(|(rest, value)| (rest, value.iter().map(|v| *v == "#").collect_vec()))
}

/// Parses [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
fn parse_machine(input: &str) -> IResult<&str, Input> {
    all_consuming((parse_leds, tag(" "), parse_buttons, tag(" "), delimited(tag("{"), parse_number_list, tag("}")))).parse(input)
        .map(|(rest, (leds, _, buttons, _, joltages))| (rest, Input {
            leds: leds,
            buttons: buttons,
            joltages: joltages,
        }))
}

fn parse_input(lines: &Vec<String>) -> crate::helpers::Result<Vec<Input>> {
    let mut inputs: Vec<Input> = Vec::new();

    for (_idx, line) in lines.iter().enumerate() {
        let (_, input) = parse_machine(line.as_str()).unwrap();
        inputs.push(input);
    }

    Ok(inputs)
}

fn solve_machine(machine: &Input) -> i64 {
    let start_leds: Vec<bool> = vec![false; machine.leds.len()];
    let mut queue: VecDeque<(i64, Vec<bool>)> = VecDeque::new();
    let mut visited: HashSet<Vec<bool>> = HashSet::new(); // All nodes that were put into queue at any point.
    queue.push_back((0, start_leds.clone()));
    visited.insert(start_leds);
    loop {
        if let Some((rank, leds)) = queue.pop_front() {
            if leds == machine.leds {
                return rank;
            }

            for button in &machine.buttons {
                let mut new_leds = leds.clone();
                for &led in button {
                    new_leds[led as usize] = !new_leds[led as usize];
                }

                if visited.contains(&new_leds) {
                    continue;
                }

                queue.push_back((rank + 1, new_leds.clone()));
                visited.insert(new_leds);
            }
        } else {
            panic!("Did not find any solution.")
        }
    }
}

#[allow(dead_code)]
fn solve_task(inputs: &Vec<Input>) -> i64 {
    let mut result = 0;
    for input in inputs {
        result += solve_machine(input);
    }
    return result;
}

fn solve_machine2(machine: &Input) -> i64 {
    let num_counters = machine.joltages.len();

    let mut counter_to_buttons: Vec<Vec<usize>> = vec![vec![]; num_counters]; // For each counter we collect indices of buttons that affect it.
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            counter_to_buttons[counter_idx as usize].push(button_idx);
        }
    }
    let max_button_for_counters = counter_to_buttons.iter().map(|v| *v.iter().max().unwrap()).collect_vec();

    let start_counters: Vec<i64> = vec![0; machine.joltages.len()];
    let mut queue: VecDeque<(i64, usize, Vec<i64>)> = VecDeque::new();
    let mut visited: HashSet<Vec<i64>> = HashSet::new(); // All nodes that were put into queue at any point.
    queue.push_back((0, 0, start_counters.clone()));
    visited.insert(start_counters);
    loop {
        if let Some((rank, last_button_pressed, counters)) = queue.pop_front() {
            if counters == machine.joltages {
                return rank;
            }

            let mut counter_to_settle = 0;
            loop {
                if counters[counter_to_settle] != machine.joltages[counter_to_settle] {
                    break;
                }
                counter_to_settle += 1;
            }

            let max_button_for_counter = max_button_for_counters[counter_to_settle];
            if last_button_pressed > max_button_for_counter {
                continue;
            }

            'next_button:
            //for &button_idx in &counter_to_buttons[counter_to_settle] { // We don't allow to press previous buttons.
            for button_idx in last_button_pressed..machine.buttons.len() { // We don't allow to press previous buttons.
                let mut new_counters = counters.clone();
                let button = &machine.buttons[button_idx];
                for &idx in button {
                    new_counters[idx as usize] += 1;
                    if new_counters[idx as usize] > machine.joltages[idx as usize] {
                        continue 'next_button;
                    }
                }

                let key = new_counters.clone();
                if visited.contains(&key) {
                    continue;
                }

                queue.push_back((rank + 1, button_idx, new_counters));
                visited.insert(key);
            }
        } else {
            panic!("Did not find any solution.")
        }
    }
}

fn solve_machine3(machine: &Input) -> i64 {
    let num_counters = machine.joltages.len();

    let mut counter_to_buttons: Vec<Vec<usize>> = vec![vec![]; num_counters]; // For each counter we collect indices of buttons that affect it.
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            counter_to_buttons[counter_idx as usize].push(button_idx);
        }
    }

    for (counter_idx, buttons) in counter_to_buttons.iter().enumerate() {
        for &button_idx in buttons {
            print!("b{} + ", button_idx)
        }
        println!("0 = {}", machine.joltages[counter_idx]);
    }

    0
}

#[allow(dead_code)]
fn solve_task2(inputs: &Vec<Input>) -> i64 {
    let mut result = 0;
    for input in inputs {
        let res = solve_machine3(input);
        println!("Machine: {}", res);
        result += res;
    }
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec10() {
    let lines = read_lines("dec10.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task(&inputs);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec10_2() {
    let lines = read_lines("dec10.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task2(&inputs);
    println!("{:?}", result);
}
