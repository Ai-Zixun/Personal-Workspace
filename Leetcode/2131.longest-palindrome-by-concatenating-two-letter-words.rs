/*
 * @lc app=leetcode id=2131 lang=rust
 *
 * [2131] Longest Palindrome by Concatenating Two Letter Words
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut counter = vec![vec![0; 26]; 26];
        let mut res = 0;

        for word in words.iter() {
            let word: Vec<char> = word.chars().collect();
            let a = word[0] as usize - 'a' as usize;
            let b = word[1] as usize - 'a' as usize;

            if counter[b][a] > 0 {
                counter[b][a] -= 1;
                res += 4;
            } else {
                counter[a][b] += 1;
            }
        }

        for i in 0..26 {
            if counter[i][i] > 0 {
                return res + 2;
            }
        }

        res
    }
}
// @lc code=end
