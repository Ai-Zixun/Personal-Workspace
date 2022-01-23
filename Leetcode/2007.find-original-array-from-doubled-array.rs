/*
 * @lc app=leetcode id=2007 lang=rust
 *
 * [2007] Find Original Array From Doubled Array
 */

// @lc code=start
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        if changed.len() % 2 != 0 {
            return vec![];
        }

        let mut changed = changed.clone();
        changed.sort();

        let mut original: Vec<i32> = vec![];

        let mut element_indexes: HashMap<i32, usize> = HashMap::new();
        changed.iter().enumerate().for_each(|(idx, val)| {
            *element_indexes.entry(*val).or_insert(0) += 1;
        });

        print!("{:?}", element_indexes);

        for value in changed.into_iter() {
            let count = *element_indexes.get(&value).unwrap();
            if count != 0 {
                original.push(value);
                let double = value * 2;
                if element_indexes.contains_key(&double) && element_indexes[&double] > 0 {
                    element_indexes.insert(double, element_indexes[&double] - 1);
                } else {
                    return vec![]
                }

                let count = *element_indexes.get(&value).unwrap();
                element_indexes.insert(value, count - 1);
            }
        }

        print!("{:?}", element_indexes);

        original
    }
}
// @lc code=end
