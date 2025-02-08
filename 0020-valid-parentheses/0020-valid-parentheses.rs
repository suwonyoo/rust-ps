impl Solution {
    pub fn is_match(first: char, second: char) -> bool {
        (first == '{' && second == '}') ||
        (first == '(' && second == ')') ||
        (first == '[' && second == ']')
    }

    pub fn is_right(input: char) -> bool {
        input == '(' || input == '{' || input == '['
    }

    pub fn is_valid(s: String) -> bool {
            // 인덱스 받아서 
        let mut stack : Vec<char> = Vec::new();

        if s.len() % 2 == 1 {
            return false
        }


        // 반복문 돌리면서
        for (index, value) in s.chars().enumerate(){
            if Solution::is_right(value) {
            // 오른쪽으로 열렸으면
                stack.push(value);
            
            }else {
                if stack.len()==0 {
                    return false
                }

            // 왼쪽으로 열렸으면
                if !Solution::is_match(stack.pop().unwrap(), value) {
                    return false
                } 
            }
        }

        if stack.len() == 0{
            return true
        }
        false
    }
}


