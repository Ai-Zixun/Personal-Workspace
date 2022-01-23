/*
 * @lc app=leetcode id=2122 lang=rust
 *
 * [2122] Recover the Original Array
 */

// @lc code=start
impl Solution {
    fn check_valid(nums: &Vec<i32>, k: i32) -> (bool, Vec<i32>) {
        use std::collections::HashMap;

        let mut counts: HashMap<i32, i32> = HashMap::new();
        nums.clone()
            .into_iter()
            .for_each(|item| *counts.entry(item).or_default() += 1);

        let mut ans = vec![];
        for num in nums.iter() {
            if counts.get_mut(&num).cloned().unwrap_or(0) == 0 {
                continue;
            } else if counts.get_mut(&(num + k)).cloned().unwrap_or(0) == 0 {
                return (false, vec![]);
            } else {
                *counts.get_mut(&num).unwrap() -= 1;
                *counts.get_mut(&(num + k)).unwrap() -= 1;
                ans.push(num + k / 2);
            }
        }

        (true, ans)
    }

    pub fn recover_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let len = nums.len();

        for i in 1..len {
            let k = nums[i] - nums[0];
            if k != 0 && k % 2 == 0 {
                let (valid, res) = Solution::check_valid(&nums, k);
                if valid {
                    return res;
                }
            }
        }

        nums
    }
}
// @lc code=end
