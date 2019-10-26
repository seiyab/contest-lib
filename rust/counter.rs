use std::hash::Hash;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::cmp::Ordering;

type Graph<N> = HashMap<N, BTreeMap<N, u64>>;

#[derive(PartialEq, Eq)]
struct RevOrd<M: Ord + Eq>(M);

#[allow(dead_code)]
impl <M: Ord + Eq> PartialOrd for RevOrd<M> {
    fn partial_cmp(&self, other: &RevOrd<M>) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

#[allow(dead_code)]
impl <M: Ord + Eq> Ord for RevOrd<M> {
    fn cmp(&self, other: &RevOrd<M>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[allow(dead_code)]
struct MinHeap<M: Ord>(BinaryHeap<RevOrd<M>>);

#[allow(dead_code)]
impl <M: Ord> MinHeap<M> {
    fn new() -> MinHeap<M> {
        MinHeap(BinaryHeap::new())
    }

    fn push(&mut self, elm: M) {
        self.0.push(RevOrd(elm))
    }

    fn pop(&mut self) -> Option<M> {
        match self.0.pop() {
            Some(RevOrd(elm)) => Some(elm),
            _ => None
        }
    }
}

fn dijkstra<N>(graph: &Graph<N>, start: N, end: N) -> Option<u64>
    where N: Eq + Ord + Hash + Copy
{
    let mut heap = MinHeap::new();
    let mut visited = HashSet::new();

    heap.push((0, start));

    while let Some((cost, current)) = heap.pop() {
        if visited.contains(&current) { continue; }
        if current == end { return Some(cost); }
        visited.insert(current);
        if let Some(edges) = graph.get(&current) {
            for (next, path_cost) in edges {
                heap.push((cost + *path_cost, next.clone()))
            }
        }
    }
    None
}
