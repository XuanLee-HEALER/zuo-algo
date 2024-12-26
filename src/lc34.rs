// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.

// If target is not found in the array, return [-1, -1].

// You must write an algorithm with O(log n) runtime complexity.
struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            vec![-1, -1]
        } else {
            vec![
                Self::search_max(&nums, target),
                Self::search_min(&nums, target),
            ]
        }
    }

    // 搜索小于等于target的最右位置
    fn search_min(nums: &[i32], target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut res = -1;
        while l <= r {
            let m = l + ((r - l) >> 1);
            if nums[m] <= target {
                if nums[m] == target {
                    res = m as i32;
                }
                l = m + 1;
            } else if m > 0 {
                r = m - 1;
            } else {
                break;
            }
        }
        res
    }

    // 搜索大于等于target的最左位置
    fn search_max(nums: &[i32], target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut res = -1;
        while l <= r {
            let m = l + ((r - l) >> 1);
            if nums[m] >= target {
                if nums[m] == target {
                    res = m as i32;
                }
                if m > 0 {
                    r = m - 1;
                } else {
                    break;
                }
            } else {
                l = m + 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test_solution {

    #[test]
    fn test_search_range() {
        // [5,7,7,8,8,10], target = 8 3,4
        // [5,7,7,8,8,10], target = 6 -1,-1
        let sample = [(vec![5, 7, 7, 8, 8, 10], 8), (vec![5, 7, 7, 8, 8, 10], 6)];
        assert_eq!(
            super::Solution::search_range(sample[0].0.clone(), sample[0].1),
            vec![3, 4]
        );
        assert_eq!(
            super::Solution::search_range(sample[1].0.clone(), sample[1].1),
            vec![-1, -1]
        );
    }
}
