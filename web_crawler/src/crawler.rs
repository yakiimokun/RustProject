use std::borrow::Borrow;
use std::hash::Hash;
use std::collections::{VecDeque, HashSet};

// define common behavior
pub trait AdjacentNodes {
    type Node;

    // @input v (Node)
    // @return adjacentNode of v Vec<Self::Node>
    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node>;
}

// 'a means life time to specify lifetime of reference
pub struct Crawler<'a, G: AdjacentNodes> {
    graph: &'a G,
    visit: VecDeque<<G as AdjacentNodes>::Node>,
    visited: HashSet<<G as AdjacentNodes>::Node>
}

// trait border
// Clone crate: return vertex Iterator which pop from "visit" and push "visited"
//              because "visited" needs to own vertex and move ownership from "visited"  
//              to calling function
// Hash, Eq, Borrow: to call Hashset method
impl<'a, G> Crawler<'a, G>
where G: AdjacentNodes,
    <G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node> {
    pub fn new(graph: &'a G, start: <G as AdjacentNodes>::Node) -> Self {
        let mut visit = VecDeque::new(); // list of page to visit
        let visited   = HashSet::new();  // list of visited page

        visit.push_back(start);

        Self {
            graph: graph,
            visit: visit,
            visited: visited,
        }
    }
}

impl<'a, G> Iterator for Crawler<'a, G>
where G: AdjacentNodes,
<G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node> {
    type Item = <G as AdjacentNodes>::Node;

    // @mothed pop vertex from "visit" 
    fn next(&mut self) -> Option<Self::Item> {
        // Some means binds to a valid value
        while let Some(v) = self.visit.pop_front() {
            if self.visited.contains(&v) {
                continue;
            }

            let adjacent_node = self.graph.adjacent_nodes(&v);

            // to takeOwnerShip node into_iter
            for node in adjacent_node.into_iter() {
                if !self.visited.contains(&node) {
                    self.visit.push_back(node);
                }
            }

            self.visited.insert(v.clone());

            return Some(v);
        }

        None
    }
}

// it works when user enters "cargo test"
#[cfg(test)]
mod test {
    use super::*; // it means that we can use external module form internal module

    struct AdjVec(Vec<Vec<usize>>);

    impl AdjacentNodes for AdjVec {
        // same size of this machine's pointer
        type Node = usize;

        fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
            self.0.get(*v) // top index of 
                .cloned()              // copyed Vec<usize> in Option
                .unwrap_or(Vec::new()) // if vertex is None
        }
    }

    #[test]
    fn bfs() {
        let graph = AdjVec(vec![
            vec![1, 2],
            vec![0, 3],
            vec![3],
            vec![2, 0]
        ]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<usize> = crawler.collect();

        assert_eq!(nodes, vec![0, 1, 2, 3]);
    }

    #[test]
    fn bfs2() {
        let graph = AdjVec(vec![
            vec![1],
            vec![0,2,4],
            vec![0,3],
            vec![0],
            vec![0]
        ]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<usize> = crawler.collect();

        assert_eq!(nodes, vec![0, 1, 2, 4, 3]);
    }
}

