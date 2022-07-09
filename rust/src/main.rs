mod problems;
use std::collections::BinaryHeap;

use problems::solution::Solution;

fn main() {
    // let v = vec![1,2,3,4,5,6,6];
    // let res = Solution::max_area(v);
    // print!("max area of is {}", res);
    // assert_eq!(res, 49);

    let v = vec![1,1,1,15,5,6,6,6];
    let freq =Solution::top_k_frequent(v, 2);
    assert_eq!(freq, vec![6,5]);

    let mut hv = BinaryHeap::new();
    for i in vec![1,3,4,5,6,7,8,956,343,4,5,] {
        hv.push(i);
    }
    let q = hv.into_vec();
    println!("{:?}", q);
}