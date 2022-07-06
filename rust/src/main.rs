mod problems;
use problems::solution::Solution;

fn main() {
    // let v = vec![1,2,3,4,5,6,6];
    // let res = Solution::max_area(v);
    // print!("max area of is {}", res);
    // assert_eq!(res, 49);

    let v = vec![1,2,3,4,5,6,6];
    let freq =Solution::top_k_frequent(v, 2);
    assert_eq!(freq, []);
}