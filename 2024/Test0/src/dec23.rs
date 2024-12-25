use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::find_union::FindUnion;
use crate::helpers::read_lines;

fn parse_input(lines: &Vec<String>) -> Vec<(String, String)> {
    let s = "".to_string();
    let k = s.split('-');
    let muk = lines.iter().map(|s| s.split('-')
        .map(|ss| ss.to_string()).collect_tuple().expect(format!("Line {s} should have 2 elements.").as_str())).collect();
    muk
}

fn compute_ccs(edges: &Vec<(String, String)>) -> Vec<HashSet<String>> {
    let mut find_union: FindUnion<String> = FindUnion::new();
    for (u, v) in edges {
        find_union.join(u.clone(), v.clone());
    }
    find_union.get_sets()
}

type Neighbors = HashMap<String, HashSet<String>>;

fn compute_neighbors(edges: &Vec<(String, String)>) -> Neighbors {
    let mut neighbors: Neighbors = HashMap::new();
    for (u, v) in edges {
        if !neighbors.contains_key(u) {
            neighbors.insert(u.clone(), HashSet::new());
        }
        if !neighbors.contains_key(v) {
            neighbors.insert(v.clone(), HashSet::new());
        }
        neighbors.get_mut(u).unwrap().insert(v.clone());
        neighbors.get_mut(v).unwrap().insert(u.clone());
    }
    neighbors
}

fn are_neighbors(u: &str, v: &str, graph: &Neighbors) -> bool {
    graph.get(u).map_or(false, |set| set.contains(v))
}

fn compute_triples(graph: &Neighbors) -> Vec<(String, String, String)> {
    let mut result: HashSet<(String, String, String)> = HashSet::new();
    for (node, neighbors) in graph {
        if !node.starts_with('t') { continue; }
        for n in neighbors {
            let mut skip = true;
            for m in neighbors {
                if n == m { skip = false; continue; }
                if skip { continue; }
                if are_neighbors(n, m, graph) {
                    let mut v = vec![node.to_string(), n.to_string(), m.to_string()];
                    v.sort();
                    result.insert(v.iter().cloned().collect_tuple().unwrap());
                }
            }
        }
    }
    let mut res = result.iter().cloned().collect_vec();
    res.sort();
    res
}

fn find_cliques(mut candidates: HashSet<String>, mut rejected: HashSet<String>, partial_result: HashSet<String>, graph: &Neighbors, result: &mut Vec<HashSet<String>>) -> () {
    if candidates.is_empty() && rejected.is_empty() {
        result.push(partial_result);
        return;
    }

    loop {
        if candidates.is_empty() {
            return;
        }

        let candidate  = candidates.iter().next().unwrap().clone();
        let neighbors = graph.get(&candidate).unwrap();
        let mut new_partial_result = partial_result.clone();
        new_partial_result.insert(candidate.to_string());
        let new_candidates = candidates.intersection(neighbors).cloned().collect();
        let new_rejected = rejected.intersection(neighbors).cloned().collect();

        find_cliques(new_candidates, new_rejected, new_partial_result, graph, result);
        rejected.insert(candidate.to_string());
        candidates.remove(&candidate);
    }
}

#[allow(dead_code)]
pub(crate) fn dec23() {
    let lines = read_lines("dec23.in.txt").expect("Could not load input.");
    let edges = parse_input(&lines);
    let ccs = compute_ccs(&edges);
    if ccs.len() != 1 {
        panic!("Expected a single cc, but got: {}", ccs.len());
    }
    let all_nodes = ccs[0].clone();
    let graph = compute_neighbors(&edges);
    let triples = compute_triples(&graph);
    println!("{:?}", triples.len());
}

#[allow(dead_code)]
pub(crate) fn dec23_2() {
    let lines = read_lines("dec23.in.txt").expect("Could not load input.");
    let edges = parse_input(&lines);
    let ccs = compute_ccs(&edges);
    if ccs.len() != 1 {
        panic!("Expected a single cc, but got: {}", ccs.len());
    }
    let all_nodes = ccs[0].clone();
    let graph = compute_neighbors(&edges);
    let mut max_cliques: Vec<HashSet<String>> = Vec::new();
    find_cliques(all_nodes, HashSet::new(), HashSet::new(), &graph, &mut max_cliques);
    max_cliques.sort_by_key(|s| s.len());
    let best_clique = max_cliques.last().unwrap();
    let mut best_vec = best_clique.iter().cloned().collect_vec();
    best_vec.sort();
    let result = best_vec.join(",");
    println!("{:?}", result);
}
