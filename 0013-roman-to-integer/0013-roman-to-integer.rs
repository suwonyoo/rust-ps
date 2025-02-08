impl Solution {
    pub fn check_value(ss:(&char, &char), sum : i32) -> i32 {
            match ss {
                ('I', 'V') |('I', 'X' ) => sum-1,
                ('I', _) => sum+1,
                ('V', _) => sum+5,
                ('X', 'L') | ('X', 'C') => sum-10,
                ('X', _) => sum+10,
                ('L', _) => sum+50,
                ('C', 'D') | ('C', 'M')  => sum- 100,
                ('C', _) => sum+100,
                ('D', _) => sum+500,
                ('M', _) => sum+1000,
                (_, _) => sum,
            }
        }

    pub fn roman_to_int(s: String) -> i32 {
            let mut sum = 0;
            let mut last : char = ' ';

            if s.len() != 1 {
                for (ch1, ch2) in s.chars().zip(s.chars().skip(1)) {
                    sum = Solution::check_value((&ch1, &ch2), sum);
                    last = ch2
                }   
                // println!("{}", last);
                Solution::check_value((&last, &' '), sum)
            }
            else{
                let first_char = s.chars().next().unwrap(); // s의 첫 번째 문자 가져오기
                Solution::check_value((&first_char, &' '), sum)

            }

    }
}