impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let origin = x.to_string();

        let palindrome :String = origin.chars().rev().collect();

        return origin == palindrome;
    }
}