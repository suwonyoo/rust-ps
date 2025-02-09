impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result: usize = 0;
        let mut stack: Vec<char> = Vec::new();

        for character in s.chars() {
            if let Some(idx) = stack.iter().position(|&x| x == character) {
                // 중복된 문자가 나오면, 첫 등장 인덱스까지 삭제
                stack.drain(0..=idx);
            }

            stack.push(character);
            result = result.max(stack.len());
        }

        result as i32
    }
}
