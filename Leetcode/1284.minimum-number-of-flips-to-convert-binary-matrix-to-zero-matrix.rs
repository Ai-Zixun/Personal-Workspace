/*
 * @lc app=leetcode id=1284 lang=rust
 *
 * [1284] Minimum Number of Flips to Convert Binary Matrix to Zero Matrix
 */

// @lc code=start
use std::collections::{HashSet, VecDeque};

impl Solution {
    // the following answer was written by referencing to @rock bit map approach
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let row_len = mat.len();
        let col_len = mat[0].len();

        let curr = Solution::mat_to_u32(mat);
        let mut steps = 0;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(curr);
        visited.insert(curr);

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let curr = queue.pop_front().unwrap();
                // base case
                if curr == 0 {
                    return steps;
                }
                // traverse
                for row in 0..row_len {
                    for col in 0..col_len {
                        let mut next = curr;
                        Solution::mutate(&row_len, &col_len, &row, &col, &mut next);
                        if visited.insert(next) {
                            queue.push_back(next);
                        }
                    }
                }
            }

            steps += 1;
        }

        -1
    }

    fn mutate(row_len: &usize, col_len: &usize, row: &usize, col: &usize, next: &mut i32) {
        // centre
        *next ^= 1 << (row * col_len + col);
        // up
        if *row > 0 {
            *next ^= 1 << ((row - 1) * col_len + col);
        }
        // down
        if *row + 1 < *row_len {
            *next ^= 1 << ((row + 1) * col_len + col);
        }
        // left
        if *col > 0 {
            *next ^= 1 << (row * col_len + col - 1);
        }
        // right
        if *col + 1 < *col_len {
            *next ^= 1 << (row * col_len + col + 1);
        }
    }

    fn mat_to_u32(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let row_len = mat.len();
        let col_len = mat[0].len();

        for row in 0..row_len {
            for col in 0..col_len {
                res |= mat[row][col] << (row * col_len + col);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_min_flips() {
        // [[0, 0], [0, 1]] -> 3
        assert_eq!(Solution::min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
        // [[0]] -> 0
        assert_eq!(Solution::min_flips(vec![vec![0]]), 0);
        // [[1, 0, 0], [1, 0, 0]] -> -1
        assert_eq!(Solution::min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
    }

    #[test]
    fn test_mutate() {
        let mut next = 0;
        Solution::mutate(&3, &3, &1, &1, &mut next);
        assert_eq!(next, 0b010111010);

        let mut next = 0;
        Solution::mutate(&3, &3, &0, &0, &mut next);
        assert_eq!(next, 0b000001011);

        let mut next = 0;
        Solution::mutate(&3, &3, &2, &2, &mut next);
        assert_eq!(next, 0b110100000);
    }

    #[test]
    fn test_mat_to_u32() {
        // [[0, 0], [0, 1]] -> 0b1000
        assert_eq!(Solution::mat_to_u32(vec![vec![0, 0], vec![0, 1]]), 0b1000);
        // [[0, 0, 1], [0, 1, 0]] -> 0b010100
        assert_eq!(
            Solution::mat_to_u32(vec![vec![0, 0, 1], vec![0, 1, 0]]),
            0b010100
        );
        // [[0, 0, 1], [0, 1, 0], [0, 1, 0]] -> 0b010010100
        assert_eq!(
            Solution::mat_to_u32(vec![vec![0, 0, 1], vec![0, 1, 0], vec![0, 1, 0]]),
            0b010010100
        );
    }
}

// @lc code=end
