/*
 * @lc app=leetcode id=2121 lang=rust
 *
 * [2121] Intervals Between Identical Elements
 */

// @lc code=start
impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        use std::collections::HashMap;

        // For each element, record all the indexes where the same element
        // occurs
        let mut element_indexes: HashMap<i32, Vec<usize>> = HashMap::new();

        arr.iter().enumerate().for_each(|(idx, val)| {
            element_indexes.entry(*val).or_insert(Vec::new()).push(idx);
        });

        let mut res: Vec<i64> = vec![0; arr.len()];
        element_indexes.values().for_each(|indexes| {
            // prefix_sum: sum of all index diff from the starting point to the current index
            // suffix_sum: sum of all index diff from the current index to the end of the vec
            let mut prefix_sum: i64 = 0;
            let mut suffix_sum = indexes.iter().sum::<usize>() as i64;

            indexes.iter().enumerate().for_each(|(pos, index)| {
                suffix_sum -= *index as i64;
                res[*index] += suffix_sum - *index as i64 * (indexes.len() as i64 - pos as i64 - 1);
                res[*index] -= prefix_sum - *index as i64 * pos as i64;
                prefix_sum += *index as i64;
            });
        });

        res
    }
}
// @lc code=end
