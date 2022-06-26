


pub fn next_permutation(nums: &mut Vec<i32>) {
    
    let length = nums.len();
    let (mut i, mut j) = (length -1, length - 1);

    while i > 0 {
        if nums[i] > nums [i-1] {
            break;
        }
        i-=1;
    }
    
    if i == 0 {
        nums.reverse();
        return
    }

    while j> i-1 {
        if nums[j]> nums[i-1] {
            break;
        }
        j-=1
    }
    nums.swap(j, i-1);

}
