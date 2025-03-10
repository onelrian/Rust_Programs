
pub fn wiggle_sort(nums: &mut Vec<i32>) {
    let len = nums.len();
    for i in 0..len {
        if i % 2 == 1 {
            if nums[i] < nums[i - 1] {
                nums.swap(i, i - 1);
            }
        } else if i != 0 && nums[i] > nums[i - 1] {
            nums.swap(i, i - 1);
        }
    }
}
