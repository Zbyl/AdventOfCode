use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub(crate) struct FindUnion<T: Clone + PartialEq + Eq + Hash> {
    nodes: HashMap<T, i32>,
    rev_nodes: HashMap<i32, T>,
    parents: HashMap<i32, i32>,
}

#[allow(dead_code)]
impl<T: Clone + PartialEq + Eq + Hash> FindUnion<T> {
    pub fn new() -> Self
    {
        Self { nodes: HashMap::new(), rev_nodes: HashMap::new(),  parents: HashMap::new() }
    }

    pub fn _get_node(&mut self, u: T) -> i32 {
        if self.nodes.contains_key(&u) {
           return *self.nodes.get(&u).unwrap();
        }

        let idx = self.nodes.len() as i32;
        self.nodes.insert(u.clone(), idx);
        self.rev_nodes.insert(idx, u);
        self.parents.insert(idx, -1);
        idx
    }

    pub fn _find_root(&self, start_idx: i32) -> i32 {
        let mut idx = start_idx;
        loop {
            let prev_idx = idx;
            idx = *self.parents.get(&idx).unwrap();
            if idx == -1 {
                return prev_idx;
            }
        }
    }

    pub fn contains(&self, u: T) -> bool {
        self.nodes.contains_key(&u)
    }

    pub fn ensure(&mut self, u: T) {
        self._get_node(u);
    }

    pub fn join(&mut self, u: T, v: T) {
        let nu = self._get_node(u);
        let nv = self._get_node(v);
        let ru = self._find_root(nu);
        let rv = self._find_root(nv);
        self.parents.insert(ru, rv);
    }

    pub fn find_root(&mut self, u: T) -> T {
        let idx = self._get_node(u);
        let root_idx = self._find_root(idx);
        self.rev_nodes.get(&root_idx).unwrap().clone()
    }

    pub fn get_sets(&self) -> Vec<HashSet<T>> {
        let mut roots = HashMap::<i32, Vec<i32>>::new();
        for idx in 0..self.nodes.len() as i32 {
            let root = self._find_root(idx);
            let nodes = roots.entry(root).or_insert(Vec::new());
            nodes.push(idx);
        }

        let mut result: Vec<HashSet<T>> = Vec::new();
        for (_root, nodes) in roots {
            result.push(nodes.into_iter().map(|idx| self.rev_nodes.get(&idx).unwrap().clone()).collect());
        }
        result
    }
}
