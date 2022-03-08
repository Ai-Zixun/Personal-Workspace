/*
 * @lc app=leetcode id=310 lang=rust
 *
 * [310] Minimum Height Trees
 */

// @lc code=start
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashSet, VecDeque};
        let mut n = n as usize;
        let mut adjacent_nodes: Vec<HashSet<usize>> = vec![HashSet::new(); n];

        if n == 1 {
            return vec![0];
        }


        for edge in edges.iter() {
            adjacent_nodes[edge[0] as usize].insert(edge[1] as usize);
            adjacent_nodes[edge[1] as usize].insert(edge[0] as usize);
        }

        let mut leaves = vec![];

        for node in 0..(n) {
            if adjacent_nodes[node].len() == 1 {
                leaves.push(node);
            }
        }

        while n > 2 {
            n -= leaves.len();
            let mut new_leaves = vec![];
            for i in 0..leaves.len() {
                let leaf = leaves[i];
                let parent = (*adjacent_nodes[leaf].iter().next().unwrap()).clone();
                adjacent_nodes[parent].remove(&leaf);
                if adjacent_nodes[parent].len() == 1{
                    new_leaves.push(parent);
                }
            }
            leaves = new_leaves;
        }

        leaves.into_iter().map(|leaf| leaf as i32).collect()
    }
}
// @lc code=end
