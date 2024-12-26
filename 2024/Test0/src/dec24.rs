use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use itertools::Itertools;
use rand::Rng;
use regex::Regex;
use crate::dec7::Task;
use crate::helpers::{read_lines, separate_by_blank};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Op {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Gate {
    a: String,
    b: String,
    op: Op,
}

#[derive(Debug, Clone)]
struct Circuit {
    states: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

fn parse_input(lines: &Vec<String>) -> Circuit {
    let (lines0, lines1) = separate_by_blank(lines);

    let mut circuit = Circuit {
        states: HashMap::new(),
        gates: HashMap::new(),
    };

    let state_regex = Regex::new(r"^(?<name>[a-zA-Z0-9]+): (?<value>[01])$").unwrap();
    let gate_regex = Regex::new(r"^(?<a>[a-zA-Z0-9]+) (?<op>AND|OR|XOR) (?<b>[a-zA-Z0-9]+) -> (?<c>[a-zA-Z0-9]+)$").unwrap();
    for line in lines0 {
        let Some(caps) = state_regex.captures(&line) else {
            panic!("Line {} does not match the state regex.", line);
        };

        circuit.states.insert(caps["name"].to_string(), &caps["value"] == "1");
    }

    for line in lines1 {
        let Some(caps) = gate_regex.captures(&line) else {
            panic!("Line {} does not match the gate regex.", line);
        };

        let gate = Gate {
            a: caps["a"].to_string(),
            b: caps["b"].to_string(),
            op: match &caps["op"] {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => unreachable!(),
            }
        };
        circuit.gates.insert(caps["c"].to_string(), gate);
    }

    circuit
}

fn topo_dfs<'a, T>(node: &'a T, neighbors: &HashMap<&T, Vec<&'a T>>, visited: &mut HashSet<&'a T>, result: &mut Vec<T>) -> ()
where
    T: Eq + Hash + Clone,
{
    if visited.contains(node) { return; }
    visited.insert(node);

    for &neighbor in neighbors.get(node).unwrap_or(&vec![]) {
        topo_dfs(neighbor, neighbors, visited, result)
    }

    result.push(node.clone());
}

// Topology Sort
// Edge x -> y means that x will be first in the result.
fn topo_sort<T>(nodes: &HashSet<T>, edges: &HashSet<(T, T)>) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut neighbors: HashMap<&T, Vec<&T>> = HashMap::new(); // Edge x-> y will result in y being in neighbors of x.
    for (x, y) in edges {
        neighbors.entry(x).or_insert(Vec::new()).push(y);
    }

    let mut result: Vec<T> = Vec::new();

    let mut visited: HashSet<&T> = HashSet::new();
    for start_node in nodes {
        topo_dfs(start_node, &neighbors, &mut visited, &mut result);
    }

    result.reverse();
    result
}

fn gates_order(circuit: &Circuit) -> Vec<String> {
    let nodes: HashSet<String> = circuit.gates.keys().cloned().collect();
    let edges: HashSet<(String, String)> = circuit.gates.iter().map(|(out, gate)| [(gate.a.clone(), out.clone()), (gate.b.clone(), out.clone())]).flatten().collect();
    let topo = topo_sort(&nodes, &edges);
    topo
}

fn print_graph(circuit: &Circuit) {
    let nodes: HashSet<String> = circuit.gates.keys().cloned().collect();
    let edges: HashSet<(String, String)> = circuit.gates.iter().map(|(out, gate)| [(gate.a.clone(), out.clone()), (gate.b.clone(), out.clone())]).flatten().collect();

    println!("digraph Circuit {{");

    for (node, gate) in circuit.gates.iter() {
        let c = match gate.op {
            Op::AND => "red",
            Op::OR => "green",
            Op::XOR => "yellow",
        };
        println!("  {} [color={:?}];", node, c);
    }

    println!();

    for (a, b) in edges {
        println!("  {} -> {};", a, b);
    }

    println!("}}");
}


fn decode(prefix: &str, states: &HashMap<String, bool>) -> i64 {
    let mut result: i64 = 0;
    for (name, value) in states {
        if !value { continue; }
        let Some(bit_num) = name.strip_prefix(prefix) else {
            continue;
        };
        let bit_num: i64 = bit_num.parse().unwrap();
        result |= 1i64 << bit_num;
    }
    result
}

fn inject(prefix: &str, value: i64, states: &mut HashMap<String, bool>) {
    for bit_num in 0..64 {
        let val = (value & (1i64 << bit_num)) != 0;
        states.insert(format!("{}{:02}", prefix, bit_num), val);
    }
}

fn compute(circuit: &Circuit) -> i64 {
    let gate_order = gates_order(&circuit);
    println!("{:?}", gate_order);

    let mut states: HashMap<String, bool> = circuit.states.clone();

    //let x = decode("x", &states);
    //let y = decode("y", &states);

    let mut rng = rand::thread_rng(); // Initialize the random number generator
    let x: i64 = rng.random_range(1..=1i64 << 46); // Range includes 1 to 10
    let y: i64 = rng.random_range(1..=1i64 << 46); // Range includes 1 to 10
    let z = x + y;
    inject("x", x, &mut states);
    inject("y", y, &mut states);

    for name in gate_order {
        if states.contains_key(&name) { continue; }
        let gate = circuit.gates.get(&name).unwrap();
        let Some(&a) = states.get(&gate.a) else {
            println!("Cannot compute {} because {} doesn't have a value.", name, gate.a);
            continue;
        };
        let Some(&b) = states.get(&gate.b) else {
            println!("Cannot compute {} because {} doesn't have a value.", name, gate.b);
            continue;
        };
        let res = match gate.op {
            Op::AND => a && b,
            Op::OR => a || b,
            Op::XOR => a != b,
        };

        println!("Computed {}: {}", name, res);
        if let Some(bit_num) = name.strip_prefix("z") {
            let bit_num: i64 = bit_num.parse().unwrap();
            let wanted_bit = (z & (1i64 << bit_num)) != 0;
            if wanted_bit != res {
                println!("  -- Bad value for {}. Expected: {}", name, wanted_bit);
            }
        };

        states.insert(name, res);
    }

    println!("{} + {} = {}", x, y, z);

    let actual_z = decode("z", &states);
    actual_z
}

#[allow(dead_code)]
pub(crate) fn dec24() {
    let topo = topo_sort(&hashset! {"a", "b", "c", "d", "e", "f"}, &hashset! {("a", "b"), ("a", "c"), ("b", "d"), ("d", "e"), ("c", "e")});
    println!("{:?}", topo);

    let lines = read_lines("dec24.in2.txt").expect("Could not load input.");
    let circuit = parse_input(&lines);
    let result = compute(&circuit);
    println!("{:?}", result);
    //print_graph(&circuit);
}