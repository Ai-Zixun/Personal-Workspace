use std::collections::VecDeque;

/*
 * @lc app=leetcode id=1591 lang=rust
 *
 * [1591] Strange Printer II
 */

// @lc code=start
impl Solution {
    pub fn is_printable(mut target_grid: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; 61];
        let mut indegrees = vec![0 as usize; 61];

        for color in 1..=60 {
            Solution::search(&target_grid, color, &mut graph, &mut indegrees)
        }

        let mut deque = std::collections::VecDeque::<usize>::new();
        let mut seen = std::collections::HashSet::<usize>::new();

        for i in 0..61 {
            if indegrees[i] == 0 {
                deque.push_back(i);
            }
        }

        while !deque.is_empty() {
            let curr = deque.pop_front().unwrap();

            if !seen.insert(curr) {
                continue;
            }

            for neighbor in &graph[curr] {
                indegrees[*neighbor] -= 1;
                if indegrees[*neighbor] == 0 {
                    deque.push_back(*neighbor);
                }
            }
        }

        seen.len() == 61
    }

    fn search(
        grid: &Vec<Vec<i32>>,
        color: i32,
        mut graph: &mut Vec<Vec<usize>>,
        mut indegrees: &mut Vec<usize>,
    ) {
        let mut x_min = usize::MAX;
        let mut x_max = usize::MIN;
        let mut y_min = usize::MAX;
        let mut y_max = usize::MIN;

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == color {
                    x_min = std::cmp::min(x_min, x);
                    x_max = std::cmp::max(x_max, x);
                    y_min = std::cmp::min(y_min, y);
                    y_max = std::cmp::max(y_max, y);
                }
            }
        }

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                if grid[x][y] != color {
                    graph[grid[x][y] as usize].push(color as usize);
                    indegrees[color as usize] += 1;
                }
            }
        }
    }
}
// @lc code=end
