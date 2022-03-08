/*
 * @lc app=leetcode id=1857 lang=rust
 *
 * [1857] Largest Color Value in a Directed Graph
 */

// @lc code=start
use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let node_count = colors.len();
        let chars: Vec<usize> = colors.chars().map(|c| c as usize - 'a' as usize).collect();
        // Covert the graph to adjacency list format
        let (graph, mut indegrees) = {
            let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
            let mut indegrees: Vec<usize> = vec![0; node_count];

            for edge in edges.into_iter() {
                let u = edge[0] as usize;
                let v = edge[1] as usize;

                graph.entry(u).or_insert(HashSet::new()).insert(v);
                indegrees[v] += 1;
            }

            (graph, indegrees)
        };

        // Conduct topological sort
        let mut freq: Vec<Vec<usize>> = vec![vec![0; 26]; node_count];
        let mut queue = VecDeque::<usize>::new();
        for (idx, &indegree) in indegrees.iter().enumerate() {
            if indegree == 0 {
                queue.push_back(idx);
                freq[idx][chars[idx]] += 1;
            }
        }

        let mut res = 0;
        let mut processed = 0;

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            res = cmp::max(res, *freq[curr].iter().max().unwrap());

            if let Some(next_set) = graph.get(&(curr)) {
                for &next in next_set.iter() {
                    for i in 0..26 {
                        let count = if chars[next] == i { 1 } else { 0 };
                        freq[next][i] = cmp::max(freq[next][i], freq[curr][i] + count)
                    }

                    indegrees[next] -= 1;
                    if indegrees[next] == 0 {
                        queue.push_back(next);
                    }
                }
            }

            processed += 1;
        }

        if processed == node_count {
            res as i32
        } else {
            -1
        }
    }
}
// @lc code=end
