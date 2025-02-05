impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        
    let original_number : i32 = x;
    if original_number < 0 {
        false
    }
    else if original_number == 0 {
        true
    } 
    else {
        let str_original_number : String = original_number.to_string();
        let reverse : String = str_original_number
        .chars()
        .rev()
        .collect();

        if str_original_number == reverse {
            true
        } 
        else {
            false
        }
  
    }
   
    // let string_original_number = String::from(original_number);



    }
}