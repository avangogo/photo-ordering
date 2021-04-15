use itertools::Itertools;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("No input found");
    let (graph, max_by_page) = read_file(&filename).unwrap();
    match min_pages(graph, max_by_page) {
        Some(n_pages) => println!("{}", n_pages),
        None => println!("Impossible"),
    }
}

/// Compute the minimum number of pages given the image graph and the max on each page.
fn min_pages(graph: DependencyGraph, max_by_page: usize) -> Option<usize> {
    assert!(max_by_page > 0);
    if !graph.is_acyclic() { None }
    else {
        Some(min_pages_feasible(graph, max_by_page))
    }
}

/// Return the minimum number of pages assuming the problem is feasible.
fn min_pages_feasible(mut graph: DependencyGraph, max_by_page: usize) -> usize {
    let n_photos = graph.count_vertices();
    if n_photos == 0 {
        return 0;
    };
    if max_by_page == 1 {
        return n_photos;
    }
    // Shortcut for when the edge density is low enough to ensure that a greedy approach
    // gives ceil(n_photos / max_by_page) pages.
    if n_photos >= max_by_page * (graph.count_edges() + 1) {
        return (n_photos + max_by_page - 1) / max_by_page;
    };
    // Get the photos that can go on the next page
    let photos_ready = graph.get_roots();
    assert!(!photos_ready.is_empty());
    // Case 1: All ready-to-use photos fit in the next page
    if photos_ready.len() <= max_by_page {
        for &photo in &photos_ready {
            graph.remove(photo);
        }
        1 + min_pages_feasible(graph, max_by_page)
    } else {
        // Case 2: Try all max_by_page-combination for the next page
        let mut result = n_photos;
        for page in photos_ready.iter().combinations(max_by_page) {
            let mut subgraph = graph.clone();
            for &photo in page {
                subgraph.remove(photo);
            }
            result = min(
                result,
                1 + min_pages_feasible(subgraph, max_by_page),
            );
        }
        result
    }
}

/// Directed graph data structure by adjacency lists
#[derive(Clone, Debug, PartialEq, Eq)]
struct DependencyGraph {
    adj_list: BTreeMap<u32, Vec<u32>>,
}

impl DependencyGraph {
    fn new(edges: Vec<(u32, u32)>, n_vertices: usize) -> Self {
        let mut adj_list = BTreeMap::new();
        for v in 1..=n_vertices as u32 {
            adj_list.insert(v, Vec::new());
        }
        for (u, v) in &edges {
            adj_list.get_mut(u).unwrap().push(*v)
        }
        Self { adj_list }
    }
    /// Return the set of vertices with no ingoing edge.
    fn get_roots(&self) -> BTreeSet<u32> {
        let mut result: BTreeSet<u32> = self.adj_list.keys().copied().collect();
        for neighbourhood in self.adj_list.values() {
            for u in neighbourhood.iter() {
                result.remove(u);
            }
        }
        result
    }
    /// Remove a vertex.
    fn remove(&mut self, vertex: u32) {
        self.adj_list.remove(&vertex);
    }
    fn count_vertices(&self) -> usize {
        self.adj_list.len()
    }
    fn count_edges(&self) -> usize {
        self.adj_list.values().map(|list| list.len()).sum()
    }
    /// Compute if the graph contains no directed cycle.
    // Note: this is slower than a bfs because get_roots is not optimized.
    fn is_acyclic(&self) -> bool {
        let mut graph = self.clone();
        while !graph.adj_list.is_empty() {
            let roots = graph.get_roots();
            if roots.is_empty() {
                return false;
            } else {
                for &u in &roots {
                    graph.remove(u)
                }
            }
        }
        true
    }
}

// Reading input file
fn read_file(filename: &str) -> Result<(DependencyGraph, usize), Box<dyn Error>> {
    let mut lines = BufReader::new(File::open(filename)?).lines();
    let first_line: Vec<usize> = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let n = first_line[0];
    let m = first_line[1];
    let _k = first_line[2];
    let mut edges: Vec<(u32, u32)> = Vec::new();
    for line in lines {
        let parsed: Vec<_> = line?.split(' ').map(|s| s.parse().unwrap()).collect();
        if !parsed.is_empty() {
            edges.push((parsed[0], parsed[1]));
        }
    }
    Ok((DependencyGraph::new(edges, n), m))
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_roots() {
        let g = DependencyGraph::new(vec![(1, 4), (3, 2), (5, 3)], 6);
        let expected: BTreeSet<_> = vec![1, 5, 6].into_iter().collect();
        assert_eq!(g.get_roots(), expected);
    }
    #[test]
    fn test_count_edges() {
        let g = DependencyGraph::new(vec![(1, 4), (3, 2), (4, 3)], 4);
        assert_eq!(g.count_edges(), 3);
    }
    #[test]
    fn test_is_acyclic() {
        let c4 = DependencyGraph::new(vec![(1, 4), (4, 2), (2, 3), (3, 1)], 4);
        assert_eq!(c4.is_acyclic(), false);
        let transitive_tournament = DependencyGraph::new(vec![(1, 3), (1, 2), (2, 3)], 3);
        assert_eq!(transitive_tournament.is_acyclic(), true);
    }
    #[test]
    fn example1() {
        let g1 = DependencyGraph::new(vec![(2, 1), (3, 1), (1, 4)], 4);
        assert_eq!(min_pages(g1, 2), Some(3));
    }
    #[test]
    fn example2() {
        let g2 = DependencyGraph::new(vec![(2, 1), (3, 1), (4, 1), (1, 5)], 5);
        assert_eq!(min_pages(g2, 2), Some(4));
    }
    #[test]
    fn example3() {
        let g3 = DependencyGraph::new(vec![], 11);
        assert_eq!(min_pages(g3, 2), Some(6));
    }
    #[test]
    fn impossible_example() {
        let g = DependencyGraph::new(vec![(1, 2), (2, 3), (3, 1)], 4);
        assert_eq!(min_pages(g, 2), None);
    }
    #[test]
    fn slow_example() {
        let mut edges = Vec::new();
        for u in 1..7 {
            for v in 1..u {
                edges.push((u, v))
            }
        };
        let g = DependencyGraph::new(edges, 15);
        assert_eq!(min_pages(g, 3), Some(6));
    }
    #[test]
    fn path_example() {
        let g = DependencyGraph::new(vec![(1, 2), (2, 3), (3, 4), (4, 5)], 8);
        assert_eq!(min_pages(g, 4), Some(5));
    }
    #[test]
    fn test_load_file() {
        let g1 = DependencyGraph::new(vec![(2, 1), (3, 1), (1, 4)], 4);
        assert_eq!(
            read_file("examples/example1").expect("Error reading file"),
            (g1, 2)
        )
    }
}
