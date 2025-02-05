use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let to_check_num = target - num;

            if let Some(&j) = map.get(&to_check_num) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]

    }
}
