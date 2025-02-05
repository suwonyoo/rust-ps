impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        
        for (i, &val1) in nums.iter().enumerate() {
            for j in (i + 1)..nums.len() {
                if val1 + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
