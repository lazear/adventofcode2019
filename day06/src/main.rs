use std::collections::HashMap;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Edge {
    incoming: usize,
    outgoing: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Graph {
    nodes: HashMap<String, usize>,
    parent: HashMap<usize, usize>,
    edges: Vec<Edge>,
}

impl Graph {
    fn node_idx(&mut self, s: String) -> usize {
        let n = self.nodes.len();
        *self.nodes.entry(s).or_insert(n)
    }

    pub fn add_edge(&mut self, a: String, b: String) -> usize {
        let a_ix = self.node_idx(a);
        let b_ix = self.node_idx(b);

        let n = self.edges.len();
        self.edges.push(Edge {
            incoming: a_ix,
            outgoing: b_ix,
        });
        self.parent.insert(b_ix, a_ix);
        n
    }

    pub fn indirect_orbits_of(&self, k: &str) -> Option<usize> {
        let x = self.nodes.get(k)?;
        Some(
            IndOrbitIter {
                graph: self,
                ptr: *self.parent.get(x)?,
            }
            .count(),
        )
    }

    pub fn direct_orbit(&self, k: &str) -> Option<usize> {
        self.parent.get(self.nodes.get(k)?).map(|_| 1)
    }

    fn to_name(&self, index: usize) -> Option<&str> {
        for (s, &i) in &self.nodes {
            if i == index {
                return Some(s.as_ref());
            }
        }
        None
    }

    pub fn iter(&self, node: &str) -> Option<IndOrbitIter> {
        Some(IndOrbitIter {
            graph: self,
            ptr: *self.parent.get(self.nodes.get(node)?)?,
        })
    }
}

struct IndOrbitIter<'g> {
    graph: &'g Graph,
    ptr: usize,
}

impl<'g> Iterator for IndOrbitIter<'g> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.graph.parent.get(&self.ptr) {
            Some(id) => {
                self.ptr = *id;
                Some(self.ptr)
            }
            None => None,
        }
    }
}

fn parse<P: AsRef<Path>>(path: P) -> Option<Graph> {
    let mut g = Graph::default();
    let input = fs::read_to_string(path).ok()?;
    for line in input.lines() {
        let mut s = line.split(')');
        let a = s.next()?;
        let b = s.next()?;
        g.add_edge(a.into(), b.into());
    }
    Some(g)
}

fn part1(g: &Graph) -> Option<usize> {
    let mut s = 0;
    for node in g.nodes.keys() {
        s += g.indirect_orbits_of(node).unwrap_or(0);
        s += g.direct_orbit(node).unwrap_or(0);
    }
    Some(s)
}

fn part2(g: &Graph) -> Option<usize> {
    let mut s = 1;
    let mut shared = HashMap::new();

    for visited in g.iter("YOU")? {
        shared.insert(visited, (Some(s), None));
        s += 1;
    }
    s = 1;
    for visited in g.iter("SAN")? {
        shared.entry(visited).or_insert((None, None)).1 = Some(s);
        s += 1;
    }
    shared
        .values()
        .copied()
        .filter_map(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        })
        .min()
}

fn main() {
    let g = parse("./day06/input.txt").unwrap();
    println!("Part 1: {:?}", part1(&g));
    println!("Part 2: {:?}", part2(&g));
}

#[test]
fn test_orbits() {
    let g = parse("./test.txt").unwrap();

    assert_eq!(g.indirect_orbits_of("L"), Some(6));
    assert_eq!(g.indirect_orbits_of("K"), Some(5));
    assert_eq!(g.indirect_orbits_of("D"), Some(2));
    assert_eq!(g.indirect_orbits_of("COM"), None);
    assert_eq!(part1(&g), Some(42));
}

#[test]
fn test_orbits2() {
    let g = parse("./test2.txt").unwrap();
    assert_eq!(part2(&g), Some(4));
}
