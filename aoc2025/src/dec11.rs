use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use nom::combinator::all_consuming;
use crate::basic_parsing::read_lines;
use nom::{IResult, Parser};
use nom::{bytes::complete::tag, multi::separated_list1};
use nom::character::complete::alpha1;

#[derive(Debug, Clone)]
struct Input<'a> {
    node: &'a str,
    outputs: Vec<&'a str>,
}

/// Parses: abc abdg a
fn parse_outputs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(" "), alpha1).parse(input)
}

/// Parses: aaa: abc abdg a
fn parse_machine(input: &str) -> IResult<&str, Input> {
    all_consuming((alpha1, tag(": "), parse_outputs)).parse(input)
        .map(|(rest, (node, _, outputs))| (rest, Input {
            node: node,
            outputs: outputs,
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

fn dfs(node: &str, out_node: &str, graph: &HashMap::<&str, &Vec<&str>>) -> i64 {
    if node == out_node {
        return 1;
    }

    let mut result = 0;
    if let Some(children) = graph.get(node) {
        for &child in *children {
            result += dfs(child, out_node, graph);
        }
    }
    return result;
}

#[allow(dead_code)]
fn solve_task(inputs: &Vec<Input>, in_node: &str, out_node: &str) -> i64 {
    let mut graph = HashMap::new();
    for input in inputs {
        graph.insert(input.node, &input.outputs);
    }

    let result = dfs(in_node, out_node, &graph);
    return result;
}

fn dfs2<'a>(node: &'a str, had_dac: bool, had_fft: bool, graph: &HashMap::<&str, &Vec<&'a str>>, path_so_far: &mut HashSet<&'a str>, distance_limit: i64) -> i64 {
    if path_so_far.len() >= distance_limit as usize {
        return 0;
    }

    if node == "out" {
        return if had_dac && had_fft { 1 } else { 0 };
    }

    let have_dac = had_dac || (node == "dac");
    let have_fft = had_fft || (node == "fft");

    path_so_far.insert(node);

    let mut result = 0;
    if let Some(children) = graph.get(node) {
        for &child in *children {
            if path_so_far.contains(&child) {
                panic!("Cycle: {:?}", path_so_far);
            }
            result += dfs2(child, have_dac, have_fft, graph, path_so_far, distance_limit);
        }
    }

    path_so_far.remove(node);

    return result;
}

#[allow(dead_code)]
fn solve_task2(inputs: &Vec<Input>) -> i64 {
    let mut graph = HashMap::new();
    for input in inputs {
        graph.insert(input.node, &input.outputs);
    }
    let mut path_so_far: HashSet<&str> = HashSet::new();

    let result = dfs2("svr", false, false, &graph, &mut path_so_far, 100000);
    return result;
}

#[allow(dead_code)]
fn solve_task3(inputs: &Vec<Input>) -> i64 {
    let result0 = solve_task(&inputs, "srv", "fft") + solve_task(&inputs, "fft", "dac") + solve_task(&inputs, "dac", "out");
    let result1 = solve_task(&inputs, "srv", "dac") + solve_task(&inputs, "dac", "fft") + solve_task(&inputs, "fft", "out");
    return result0 + result1;
}

#[allow(dead_code)]
fn solve_task4(inputs: &Vec<Input>) -> i64 {
    let mut graph = HashMap::new();
    let mut all_nodes = hashset! {};
    for input in inputs {
        graph.insert(input.node, &input.outputs);
        all_nodes.insert(input.node);
        all_nodes.extend(&input.outputs);
    }

    // Memoization.
    let mut distances_init: HashMap<(&str, &str), i64> = hashmap! {};  // How many paths of at length exactly N are from A to B.
    for node in &all_nodes {
        distances_init.insert((node, node), 1); // One at most 0-length path from node to itself.
    }

    let mut distances = vec![distances_init];

    for _i in 1..all_nodes.len() { // All distances possible.
        let distances0 = distances.last().unwrap();
        println!("distances {}: {:?}", _i - 1, distances0);

        let mut distances1: HashMap<(&str, &str), i64> = hashmap! {};
        for ((node0, node1), &count0) in distances0 {
            // We limit ourselves to paths that don't go past fft or dac.
            if (node0 != node1) && ((node1 == &"fft") || (node1 == &"dac")) {
                //continue;
            }

            if count0 == 0 {
                continue;
            }
            if !graph.contains_key(node1) {
                continue;
            }
            for child in graph[node1] {
                let prev_count= distances1.get(&(node0,child)).unwrap_or(&0);
                distances1.insert((node0, child), prev_count + count0);
            }
        }

        distances.push(distances1);
    }

    let num_paths = |node0: &str, node1: &str| -> i64 {
        let key = (node0, node1);
        return (0..all_nodes.len()).map(|d| {
            let cnt = distances[d].get(&key).unwrap_or(&0);
            //println!("key={:?} d={} cnt={}", key, d, cnt);
            cnt
        }).sum();
    };

    let s_f = num_paths("svr", "fft");
    let f_d = num_paths("fft", "dac");
    let d_o = num_paths("dac", "out");
    let s_d = num_paths("svr", "dac");
    let d_f = num_paths("dac", "fft");
    let f_o = num_paths("fft", "out");

    println!("s_f={:?} f_d={:?} d_o={:?}", s_f, f_d, d_o);
    println!("s_d={:?} d_f={:?} f_o={:?}", s_d, d_f, f_o);

    let result0 = s_f * f_d * d_o;
    let result1 = s_d * d_f * f_o;
    return result0 + result1;
    //return num_paths("you", "out");
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec11() {
    let lines = read_lines("dec11.ex.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task(&inputs, "you", "out");
    //println!("{:?}", inputs);
    println!("{:?}", result);
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn dec11_2() {
    let lines = read_lines("dec11.in.txt").expect("Could not load input.");
    let inputs = parse_input(&lines).unwrap();
    let result = solve_task4(&inputs);
    println!("{:?}", result);
}
