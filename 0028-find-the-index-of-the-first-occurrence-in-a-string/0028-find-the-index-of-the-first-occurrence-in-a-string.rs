impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0; 
        }

        if let Some(idx) = haystack.find(&needle) {
            return idx as i32;
        }

        -1
        
    }
}
