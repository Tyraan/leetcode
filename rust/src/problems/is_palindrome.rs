use super::solution::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let s_count = s.len();
        let bytes = s.as_bytes();

        for i in 0..s_count {
            if bytes[i] as char != bytes[s_count - i - 1] as char {
                return false;
            }
        }
        return true;
    }
}
