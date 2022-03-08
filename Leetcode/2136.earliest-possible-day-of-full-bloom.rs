/*
 * @lc app=leetcode id=2136 lang=rust
 *
 * [2136] Earliest Possible Day of Full Bloom
 */

// @lc code=start
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut flowers: Vec<(i32, i32)> = grow_time.into_iter().zip(plant_time.into_iter()).collect();
        flowers.sort_by_key(|k| k.0);

        for (grow, plant) in flowers {
            res = std::cmp::max(res, grow) + plant;
        }

        res
    }
}
// @lc code=end
