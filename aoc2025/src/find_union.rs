use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Find-Union or Disjoint-Set data structure.
#[derive(Debug)]
pub(crate) struct FindUnion<T: Clone + PartialEq + Eq + Hash> {
    // @note It is not possible to add two nodes with the same value.
    nodes: HashMap<T, i32>,         // Map from value to node.
    rev_nodes: HashMap<i32, T>,     // Map from node to value.
    parents: HashMap<i32, i32>,     // Map from node to parent. Roots point to themselves.
    ranks: HashMap<i32, i32>,       // Map from node to rank.
}

#[allow(dead_code)]
impl<T: Clone + PartialEq + Eq + Hash> FindUnion<T> {
    /// Creates empty Find-Union structure.
    pub fn new() -> Self
    {
        Self { nodes: HashMap::new(), rev_nodes: HashMap::new(), parents: HashMap::new(), ranks: HashMap::new() }
    }

    pub fn _get_node(&mut self, u: T) -> i32 {
        if self.nodes.contains_key(&u) {
           return *self.nodes.get(&u).unwrap();
        }

        let idx = self.nodes.len() as i32;
        self.nodes.insert(u.clone(), idx);
        self.rev_nodes.insert(idx, u);
        self.parents.insert(idx, idx);
        self.ranks.insert(idx, 0);
        idx
    }

    pub fn _find_root(&mut self, start_idx: i32) -> i32 {
        let mut idx = start_idx;
        let root;
        loop {
            let prev_idx = idx;
            idx = *self.parents.get(&idx).unwrap();
            if idx == prev_idx {
                root = idx;
                break;
            }
        }

        idx = start_idx;
        loop {
            let prev_idx = idx;
            idx = *self.parents.get(&idx).unwrap();
            self.parents.insert(prev_idx, root);
            if idx == prev_idx {
                break;
            }
        }
        root
    }

    /// Returns true if u is in any set.
    /// (If u is not in any set it will NOT be added.)
    pub fn contains(&self, u: T) -> bool {
        self.nodes.contains_key(&u)
    }

    /// Make sure that u is in some set. If it is not, a new set is created for it.
    /// Returns true if u was added. False it was already in some set.
    pub fn ensure(&mut self, u: T) -> bool {
        if self.nodes.contains_key(&u) {
            return false;
        }
        self._get_node(u);
        return true;
    }

    /// Joins sets of u and v.
    /// Returns true if two sets were joined. False is they were already in the same set.
    /// (If u or v are not in any set they will be added.)
    pub fn join(&mut self, u: T, v: T) -> bool {
        let nu = self._get_node(u);
        let nv = self._get_node(v);
        let mut ru = self._find_root(nu);
        let mut rv = self._find_root(nv);

        if ru == rv { return false; }
        let rank_u = self.ranks[&ru];
        let rank_v = self.ranks[&rv];
        if rank_u < rank_v {
            (ru, rv) = (rv, ru)
        }
        self.parents.insert(rv, ru);
        if rank_u == rank_v {
            self.ranks.insert(ru, rank_u + 1);
        }
        return true;
    }

    /// Finds representant of set containing u.
    /// (If u is not in any set it will be added.)
    pub fn find_root(&mut self, u: T) -> T {
        let idx = self._get_node(u);
        let root_idx = self._find_root(idx);
        self.rev_nodes.get(&root_idx).unwrap().clone()
    }

    /// Return true if u and v are in the same set.
    /// (If u or v are not in any set they will be added.)
    pub fn connected(&mut self, u: T, v: T) -> bool {
        let nu = self._get_node(u);
        let nv = self._get_node(v);
        let ru = self._find_root(nu);
        let rv = self._find_root(nv);
        return ru == rv;
    }

    /// Returns all the sets.
    pub fn get_sets(&mut self) -> Vec<HashSet<T>> {
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
