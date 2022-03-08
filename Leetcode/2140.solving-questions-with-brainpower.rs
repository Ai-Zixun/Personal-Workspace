/*
 * @lc app=leetcode id=2140 lang=rust
 *
 * [2140] Solving Questions With Brainpower
 */

// @lc code=start
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut points: Vec<i64> = vec![0; questions.len()];
        Solution::points(&questions, 0, &mut points)
    }

    fn points(questions: &Vec<Vec<i32>>, index: usize, mut points: &mut Vec<i64>) -> i64 {
        // Exceeding the length of the vector
        if index >= questions.len() {
            return 0;
        }

        // Point has been calculated before
        if points[index] != 0 {
            return points[index];
        }

        let skip_point = Solution::points(&questions, index + 1, &mut points);
        let mut solve_point = questions[index][0] as i64;
        solve_point += Solution::points(
            &questions,
            index + 1 + questions[index][1] as usize,
            &mut points,
        );
        points[index] = std::cmp::max(skip_point, solve_point);

        points[index]
    }
}
// @lc code=end
