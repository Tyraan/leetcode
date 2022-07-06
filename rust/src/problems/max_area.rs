use std::cmp::{min, max};
use super::solution::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len()-1);
        let mut max_area  = min(height[i], height[j]) * ( j - i) as i32;
        while i < j {
            let area = min(height[i], height[j]) * ( j - i) as i32;
            max_area = max(max_area, area);
            if height[i] < height[j] {
                i +=1;
            }else{
                j -=1;
            }
        }
        max_area
    }
}
