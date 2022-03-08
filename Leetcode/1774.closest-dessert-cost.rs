/*
 * @lc app=leetcode id=1774 lang=rust
 *
 * [1774] Closest Dessert Cost
 */

// @lc code=start
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut result = i32::MAX;
        for cost in base_costs.into_iter() {
            Solution::helper(cost, &topping_costs, 0, &target, &mut result)
        }

        result
    }

    fn helper(
        current_cost: i32,
        topping_costs: &Vec<i32>,
        index: usize,
        target: &i32,
        result: &mut i32,
    ) {
        if (current_cost - target).abs() < (*result - target).abs()
            || ((current_cost - target).abs() == (*result - target).abs() && current_cost <= *target)
        {
            *result = current_cost;
        }

        if index >= topping_costs.len() || current_cost > *target {
            return;
        }

        Solution::helper(current_cost, topping_costs, index + 1, target, result);
        Solution::helper(
            current_cost + topping_costs[index],
            topping_costs,
            index + 1,
            target,
            result,
        );
        Solution::helper(
            current_cost + topping_costs[index] * 2,
            topping_costs,
            index + 1,
            target,
            result,
        );
    }
}
// @lc code=end
