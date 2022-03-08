/*
 * @lc app=leetcode id=2127 lang=rust
 *
 * [2127] Maximum Employees to Be Invited to a Meeting
 */

// @lc code=start
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut plants: Vec<(i32, i32)> = grow_time.into_iter().zip(plant_time.into_iter()).collect();
        plants.sort_by_key(|k| k.0);

        print!("{:?}", plants);

        0
    }
}
// @lc code=end
