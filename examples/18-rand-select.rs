use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let res = Solution::find_kth_largest(nums, 2);
    println!("result: {res}")
}

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let arr_len = nums.len();
        Self::kth(&mut nums, arr_len - k as usize)
    }

    fn kth(nums: &mut [i32], loc: usize) -> i32 {
        let mut rng = rand::thread_rng();
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            let ti = rng.gen_range(l..=r);
            let (tleft, tright) = Self::partition(nums, l, r, nums[ti]);
            if loc < tleft {
                r = tleft - 1;
            } else if loc > tright {
                l = tright + 1;
            } else {
                return nums[tleft];
            }
        }

        -1
    }

    fn partition(nums: &mut [i32], l: usize, r: usize, x: i32) -> (usize, usize) {
        let mut a = l;
        let mut b = r;
        let mut i = l;
        while i <= b {
            match nums[i].cmp(&x) {
                Ordering::Less => {
                    nums.swap(a, i);
                    a += 1;
                    i += 1;
                }
                Ordering::Equal => {
                    i += 1;
                }
                Ordering::Greater => {
                    nums.swap(b, i);
                    b -= 1;
                }
            }
        }

        (a, b)
    }
}
