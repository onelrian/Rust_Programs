pub struct Solution;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::new();

        for left in 0..n - 3 {
            // handling duplicate on the first number
            if left > 0 && nums[left] == nums[left - 1] {
                continue;
            }

            for mid1 in left + 1..n - 2 {
                // handling duplicate on the second number
                if mid1 > left + 1 && nums[mid1] == nums[mid1 - 1] {
                    continue;
                }

                let mut mid2 = mid1 + 1;
                let mut right = n - 1;

                while mid2 < right {
                    let total = nums[left] as i64
                        + nums[mid1] as i64
                        + nums[mid2] as i64
                        + nums[right] as i64;

                    if total == target as i64 {
                        result.push(vec![nums[left], nums[mid1], nums[mid2], nums[right]]);

                        // handling duplicate on the third number
                        while mid2 < right && nums[mid2] == nums[mid2 + 1] {
                            mid2 += 1;
                        }
                        // handling duplicate on the fourth number
                        while mid2 < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }

                        mid2 += 1;
                        right -= 1;
                    } else if total < target as i64 {
                        mid2 += 1;
                    } else {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}
