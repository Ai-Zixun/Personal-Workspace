use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Eq, PartialEq)]
struct Node {
    index: usize,
    cost: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution {}

impl Solution {
    const MODULO: i64 = 10_0000_0007;

    /// Edges: E
    /// Nodes/Vertices: V
    ///
    /// Time Complexity: O(E)
    /// Space Complexity: O(V + E)
    fn convert_edge_list_to_adjacency_list(
        edges: Vec<Vec<i32>>,
        n: usize,
    ) -> Vec<Vec<(usize, i64)>> {
        let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
        for edge in edges {
            let src = edge[0] as usize;
            let dst = edge[1] as usize;
            let cost = edge[2] as i64;
            graph[src].push((dst, cost));
            graph[dst].push((src, cost));
        }
        graph
    }

    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let graph: Vec<Vec<(usize, i64)>> = Self::convert_edge_list_to_adjacency_list(roads, n);
        let mut cost: Vec<i64> = vec![i64::MAX; n];
        let mut path_count: Vec<i64> = vec![0; n];

        // Dijkstra
        // Edges: E
        // Nodes/Vertices: V
        //
        // Time Complexity: O(E + V log V )
        // Space Complexity: O(V)
        let mut pq: BinaryHeap<Node> = BinaryHeap::new();
        path_count[0] = 1;
        cost[0] = 0;
        pq.push(Node { index: 0, cost: 0 });

        while let Some(node) = pq.pop() {
            for (next_index, next_cost) in &graph[node.index] {
                if node.cost + *next_cost > cost[*next_index] {
                    continue;
                } else if cost[*next_index] == node.cost + *next_cost {
                    path_count[*next_index] =
                        (path_count[*next_index] + path_count[node.index]) % Self::MODULO;
                } else {
                    cost[*next_index] = node.cost + *next_cost;
                    path_count[*next_index] = path_count[node.index];
                    pq.push(Node {
                        index: *next_index,
                        cost: node.cost + *next_cost,
                    });
                }
            }

            // println!("{:?} {:?} {:?}", node.index, node.cost, node.path);
            // println!("{:?}", cost);
            // println!("{:?}", path_count);
        }

        path_count[n - 1] as i32
    }
}

fn main() {
    let n = 7;
    let roads = [
        [0, 6, 7],
        [0, 1, 2],
        [1, 2, 3],
        [1, 3, 3],
        [6, 3, 3],
        [3, 5, 1],
        [6, 5, 1],
        [2, 5, 1],
        [0, 4, 5],
        [4, 6, 2],
    ];
    let roads = roads.into_iter().map(|r| r.to_vec()).collect();
    let res = Solution::count_paths(n, roads);
    println!("{:?}", res);
}
