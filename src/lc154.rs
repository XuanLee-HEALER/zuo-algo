struct Solution;

impl Solution {
    // 旋转次数就是最小值的下标
    // 一个二分查找的变体，只是因为数组中有相同元素，所以在比较时，如果nums[mid]==nums[right]，那么right这个位置的数可以去掉
    // 搜索范围变为 left..right
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[right] {
                // 最小值在 left..=mid
                right = mid;
            } else if nums[mid] > nums[right] {
                // 最小值在 mid+1..=right
                left = mid + 1;
            } else {
                // nums[mid] == nums[right]，无法判断哪边，缩掉 right
                right -= 1;
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod test_lc154 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![3, 1, 3, 3, 3]), 1);
        assert_eq!(Solution::find_min(vec![1, 1]), 1);
        assert_eq!(Solution::find_min(vec![2]), 2);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }
}
