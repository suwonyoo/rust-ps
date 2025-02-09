impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
       
        let mut target_string = s.trim();
        let mut length = target_string.len();

        if let Some(idx) = target_string.rfind(' ') {
            let result = (length - idx-1) as i32;
            return result;
        }
        length as i32
    }
}