use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd)]
struct Node {
    node: usize,
    prob: f64,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Max-heap
        self.prob
            .partial_cmp(&other.prob)
            .unwrap()
            .then_with(|| self.node.cmp(&other.node))
    }
}

struct Solution {}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start as usize;
        let end = end as usize;
        let graph = Self::convert_edge_list_to_adjacency_list(edges, succ_prob, n);

        // Dijkstra
        // Edges: E
        // Nodes/Vertices: V
        //
        // Time Complexity: O(E + V log V )
        // Space Complexity: O(V)
        let mut cost = vec![0.0; n];
        cost[start] = 1.0;

        let mut pq: BinaryHeap<Node> = BinaryHeap::new();
        pq.push(Node {
            node: start,
            prob: 1.0,
        });

        while let Some(node) = pq.pop() {
            for (next, edge_prob) in &graph[node.node] {
                let prob = *edge_prob * node.prob;
                if prob > cost[*next] {
                    cost[*next] = prob;
                    pq.push(Node { node: *next, prob })
                }
            }
        }

        cost[end]
    }

    /// Edges: E
    /// Nodes/Vertices: V
    ///
    /// Time Complexity: O(E)
    /// Space Complexity: O(V + E)
    fn convert_edge_list_to_adjacency_list(
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        n: usize,
    ) -> Vec<Vec<(usize, f64)>> {
        let mut graph: Vec<Vec<(usize, f64)>> = vec![vec![]; n];
        for i in 0..edges.len() {
            let u = edges[i][0] as usize;
            let v = edges[i][1] as usize;
            let prob = succ_prob[i];
            graph[u].push((v, prob));
            graph[v].push((u, prob));
        }
        graph
    }
}

fn main() {
    let n = 3;
    let edges = [[0, 1], [1, 2], [0, 2]];
    let edges: Vec<Vec<i32>> = edges.into_iter().map(|e| e.to_vec()).collect();
    let succ_prob = [0.5, 0.5, 0.2];
    let succ_prob: Vec<f64> = succ_prob.into_iter().collect();
    let start = 0;
    let end = 2;
    let res = Solution::max_probability(n, edges, succ_prob, start, end);
    println!("{:?}", res);
}
