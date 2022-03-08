/*
 * @lc app=leetcode id=1452 lang=rust
 *
 * [1452] People Whose List of Favorite Companies Is Not a Subset of Another List
 */

// @lc code=start
use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut superset: Vec<usize> = (0..favorite_companies.len()).collect();
        let companies: Vec<HashSet<String>> = favorite_companies
            .into_iter()
            .map(|data| HashSet::from_iter(data.into_iter()))
            .collect();

        for i in 0..companies.len() {
            for j in (i + 1)..companies.len() {
                let i_root = Solution::find_root(&mut superset, i);
                let j_root = Solution::find_root(&mut superset, j);
                if i_root != j_root {
                    if Solution::contains(&companies[i_root], &companies[j_root]) {
                        superset[j_root] = i_root;
                    }
                    if Solution::contains(&companies[j_root], &companies[i_root]) {
                        superset[i_root] = j_root;
                    }
                }
            }
        }

        let mut res = BTreeSet::new();
        for set in superset.into_iter() {
            res.insert(set as i32);
        }

        res.into_iter().collect()
    }

    fn contains(a: &HashSet<String>, b: &HashSet<String>) -> bool {
        if a.len() <= b.len() {
            return false;
        }
        b.iter().all(|item| a.contains(item))
    }

    fn find_root(superset: &mut Vec<usize>, mut index: usize) -> usize {
        while superset[index] != index {
            superset[index] = superset[superset[index]];
            index = superset[index];
        }
        index
    }
}
// @lc code=end
