use core::num;

use super::solution::Solution;


impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() || target<=nums [0] {
            return 0
        }
        if  target > nums[nums.len()] {
            return nums.len() as i32
        }
        
        let (mut leftIdx, mut rightIds, mut mid) =(0, nums.len(), nums.len()/2);

        while  {
            
        }
        3
    }
}