struct Solution;

// Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

// Return k after placing the final result in the first k slots of nums.

// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            1
        } else {
            let mut init = nums[0] - 1;
            let (mut l, mut r) = (0, 0);
            let mut count = 0;
            while r < n {
                if nums[r] == init {
                    if count < 2 {
                        count += 1;
                        nums[l] = init;
                        l += 1;
                    }
                } else {
                    count = 1;
                    init = nums[r];
                    nums[l] = init;
                    l += 1;
                }
                r += 1;
            }
            l as i32
        }
    }
}

#[cfg(test)]
mod test_solution {

    #[test]
    fn test_remove_duplicates() {
        let mut ori = vec![0, 0, 0, 1, 2, 2, 2, 3, 3, 3, 3];
        let new_len = super::Solution::remove_duplicates(&mut ori);
        assert_eq!(&ori[..new_len as usize], vec![0, 0, 1, 2, 2, 3, 3]);
    }
}
