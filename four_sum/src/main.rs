mod four_sum;
use four_sum::Solution;
fn main() {
    let nums1 = vec![3,5];
    let target1 = 15;
    println!("{:?}", Solution::four_sum(nums1, target1)); // Output: [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]

    let nums2 = vec![2, 2, 2, 2, 2];
    let target2 = 8;
    println!("{:?}", Solution::four_sum(nums2, target2)); // Output: [[2, 2, 2, 2]]
}