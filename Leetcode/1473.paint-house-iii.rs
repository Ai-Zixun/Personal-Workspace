/*
 * @lc app=leetcode id=1473 lang=rust
 *
 * [1473] Paint House III
 */

// @lc code=start
impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; n as usize + 2]; target as usize + 1]; m as usize + 1];
        for i in 1..=n {
            dp[m as usize][0][i as usize] = 0;
        }
        let ans = Solution::dfs(0, target, n + 1, &mut dp, &houses, &cost, m, n);
        if ans < std::i32::MAX {
            ans
        } else {
            -1
        }
    }

    fn dfs(
        idx: i32,
        rem: i32,
        c: i32,
        dp: &mut Vec<Vec<Vec<i32>>>,
        hs: &Vec<i32>,
        cs: &Vec<Vec<i32>>,
        m: i32,
        n: i32,
    ) -> i32 {
        if rem >= 0 && dp[idx as usize][rem as usize][c as usize] != -1 {
            return dp[idx as usize][rem as usize][c as usize];
        }
        let mut ans = std::i32::MAX;
        if idx >= m || rem < 0 {
            return if rem == 0 { 0 } else { ans };
        }
        if hs[idx as usize] > 0 {
            let diff = if hs[idx as usize] == c { 0 } else { 1 };
            dp[idx as usize][rem as usize][c as usize] =
                Solution::dfs(idx + 1, rem - diff, hs[idx as usize], dp, hs, cs, m, n);
            return dp[idx as usize][rem as usize][c as usize];
        }
        for i in 1..=n {
            let diff = if i == c { 0 } else { 1 };
            let last = Solution::dfs(idx + 1, rem - diff, i, dp, hs, cs, m, n);
            {
                if last < std::i32::MAX {
                    ans = std::cmp::min(
                        ans,
                        if i == 0 {
                            0
                        } else {
                            cs[idx as usize][i as usize - 1] + last
                        },
                    );
                }
            }
        }
        dp[idx as usize][rem as usize][c as usize] = ans;
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 0, 0, 0, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![10, 1],
                    vec![1, 10],
                    vec![5, 1]
                ],
                5,
                2,
                3
            ),
            9
        )
    }

    #[test]
    fn test_min_cost_02() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 2, 1, 2, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![10, 1],
                    vec![1, 10],
                    vec![5, 1]
                ],
                5,
                2,
                3
            ),
            11
        )
    }

    #[test]
    fn test_min_cost_03() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 0, 0, 0, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![1, 10],
                    vec![10, 1],
                    vec![1, 10]
                ],
                5,
                2,
                5
            ),
            5
        )
    }

    #[test]
    fn test_min_cost_04() {
        assert_eq!(
            Solution::min_cost(
                vec![3, 1, 2, 3],
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1],],
                4,
                3,
                3
            ),
            -1
        )
    }
}
// @lc code=end
