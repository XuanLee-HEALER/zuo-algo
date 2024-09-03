use std::{cmp, time::Instant};

use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr1: Vec<i32> = (1..=5000000).collect();
    arr1.shuffle(&mut rng);
    // println!("{arr1:?}");
    let arr2 = arr1.clone();

    let mut counter = Instant::now();
    arr1.sort();
    println!("quick sort elapsed {}ms", counter.elapsed().as_millis());
    counter = Instant::now();
    let sort_res = Solution::sort_array(arr2);
    println!("merge sort elapsed {}ms", counter.elapsed().as_millis());
    assert_eq!(sort_res, arr1);
}

struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        // Self::merge_sort_rec(&nums, 0, nums.len() - 1);
        Self::merge_sort_no_rec(&mut nums)
    }

    pub fn merge_sort_no_rec(nums: &mut [i32]) -> Vec<i32> {
        let rl = nums.len();
        let mut step = 1;
        while step < rl {
            let mut l = 0;
            while l < rl {
                let mid = l + step;
                if mid >= rl {
                    break;
                }

                let r = cmp::min(mid + step, rl);
                let left_arr = &nums[l..mid];
                let right_arr = &nums[mid..r];
                let merge_res = Self::merge(left_arr, right_arr);
                let mut tl = l;
                for e in merge_res {
                    nums[tl] = e;
                    tl += 1;
                }

                l = r;
            }
            step <<= 1;
        }

        nums.into()
    }

    pub fn merge_sort_rec(nums: &[i32], l: usize, r: usize) -> Vec<i32> {
        if l == r {
            nums[l..l + 1].into()
        } else {
            let m = l + ((r - l) >> 1);
            let left_sorted = Self::merge_sort_rec(nums, l, m);
            let right_sorted = Self::merge_sort_rec(nums, m + 1, r);
            Self::merge(&left_sorted, &right_sorted)
        }
    }

    pub fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut res = Vec::new();

        let mut i = 0;
        let mut j = 0;
        let il = left.len();
        let jl = right.len();
        while i < il && j < jl {
            if left[i] <= right[j] {
                res.push(left[i]);
                i += 1;
            } else {
                res.push(right[j]);
                j += 1;
            }
        }

        while i < il {
            res.push(left[i]);
            i += 1;
        }

        while j < jl {
            res.push(right[j]);
            j += 1;
        }

        res
    }
}
