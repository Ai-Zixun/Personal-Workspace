use std::slice::SliceIndex;

/*
 * @lc app=leetcode id=802 lang=rust
 *
 * [802] Find Eventual Safe States
 */

// @lc code=start
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet, VecDeque};

        let len = graph.len();
        let mut reverse_graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut indegrees: Vec<usize> = vec![0; len];
        let mut queue = VecDeque::<usize>::new();
        let mut res = vec![];

        // Reverse the graph
        for (v, us) in graph.iter().enumerate() {
            indegrees[v as usize] = us.len();

            if indegrees[v as usize] == 0 {
                queue.push_back(v);
            }

            for u in us {
                reverse_graph
                    .entry(*u as usize)
                    .or_insert(HashSet::new())
                    .insert(v);
            }
        }

        // Topological sort
        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            res.push(curr as i32);

            if let Some(vs) = reverse_graph.get(&(curr)) {
                for v in vs {
                    indegrees[*v] -= 1;
                    if indegrees[*v] == 0 {
                        queue.push_back(*v);
                    }
                }
            }
        }

        res.sort();

        res
    }
}
// @lc code=end
