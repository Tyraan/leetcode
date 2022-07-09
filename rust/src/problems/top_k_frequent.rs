use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};
use super::solution::Solution;


impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = HashMap::new();
        for num in nums {
            counter.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }
        // 创建一个只保存k个元素的堆，
        let mut heap =BinaryHeap::new();
        for (key,freq) in counter {
            
            if heap.len() < k as usize  {
                heap.push(Reverse((freq, key)));
            } else {
                let cnt  = heap.peek().unwrap().0 .0;
                if freq > cnt {
                    heap.pop();
                    heap.push(Reverse((freq, key)));
                }
            }
        }
        let mut res = vec![];
        while !heap.is_empty() {
            res.insert(0, heap.pop().unwrap().0.1)
        }
        res
    }
}