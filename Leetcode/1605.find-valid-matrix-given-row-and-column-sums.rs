/*
 * @lc app=leetcode id=1605 lang=rust
 *
 * [1605] Find Valid Matrix Given Row and Column Sums
 */

// @lc code=start
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; col_sum.len()]; row_sum.len()];

        for row_idx in 0..row_sum.len() {
            for col_idx in 0..col_sum.len() {
                res[row_idx][col_idx] = std::cmp::min(row_sum[row_idx], col_sum[col_idx]);
                row_sum[row_idx] -= res[row_idx][col_idx];
                col_sum[col_idx] -= res[row_idx][col_idx];
            }
        }

        res
    }
}
// @lc code=end
