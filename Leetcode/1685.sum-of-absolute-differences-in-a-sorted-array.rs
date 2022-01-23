/*
 * @lc app=leetcode id=1685 lang=rust
 *
 * [1685] Sum of Absolute Differences in a Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![0; nums.len() + 1];
        let mut sum = 0;
        (0..nums.len()).for_each(|index| {
            sum += nums[index];
            prefix[index + 1] = prefix[index] + nums[index];
        });


        let mut res = vec![0; nums.len()];
        (0..nums.len()).for_each(|index| {
            res[index] =
                nums[index] * (index as i32)
                - prefix[index] + (sum - prefix[index] - nums[index])
                - nums[index] * ((nums.len() - index - 1) as i32);
        });

        res
    }
}
// @lc code=end
