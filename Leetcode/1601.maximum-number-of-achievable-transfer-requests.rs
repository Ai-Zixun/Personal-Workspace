/*
 * @lc app=leetcode id=1601 lang=rust
 *
 * [1601] Maximum Number of Achievable Transfer Requests
 */

// @lc code=start
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut count = vec![0; n as usize];
        Solution::traverse(&requests, 0, &mut count, 0, &mut max);

        max
    }

    fn traverse(
        requests: &Vec<Vec<i32>>,
        index: usize,
        count: &mut Vec<i32>,
        move_count: i32,
        max: &mut i32,
    ) {
        // End
        if index == requests.len() {
            for c in count.iter() {
                if *c != 0 {
                    return;
                }
            }
            *max = std::cmp::max(move_count, *max);
            return;
        }

        // Move
        count[requests[index][0] as usize] += 1;
        count[requests[index][1] as usize] -= 1;
        Solution::traverse(requests, index + 1, count, move_count + 1, max);
        count[requests[index][0] as usize] -= 1;
        count[requests[index][1] as usize] += 1;


        // Not Move
        Solution::traverse(requests, index + 1, count, move_count, max)
    }
}
// @lc code=end
