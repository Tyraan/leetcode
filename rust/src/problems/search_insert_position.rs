use super::solution::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums[0] > target{
            return 0;
        }
        let n = nums.len();
        if nums[(n -1) as usize] <= target{
            return nums.len() as i32;
        }

        let mut low = 0;
        let mut high = n;
        let mut mid;

        while  low+1 < high {
            mid = (low + high) / 2;
            if nums[mid] < target {
                low = mid;
            }
            else if nums[mid] > target {
                high = mid;
            }
            else {
                return mid as i32;
            }
        }
        high as i32
    }
}