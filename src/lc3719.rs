use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut odds = HashSet::new();
        let mut evens = HashSet::new();
        let mut res: usize = 0;
        let len = nums.len();
        for i in 0..len {
            odds.clear();
            evens.clear();
            for j in i..len {
                if nums[j] % 2 != 0 {
                    odds.insert(nums[j]);
                } else {
                    evens.insert(nums[j]);
                }
                if odds.len() == evens.len() {
                    res = res.max(j - i + 1);
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod test_lc3719 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::longest_balanced(vec![1, 1, 1, 1]), 0);
        assert_eq!(Solution::longest_balanced(vec![2, 4, 6, 8]), 0);
        assert_eq!(Solution::longest_balanced(vec![1, 2]), 2);
    }
}
