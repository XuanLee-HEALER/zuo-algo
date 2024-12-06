use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let res = Solution::find_kth_largest(nums, 2);
    println!("result: {res}");
    assert_eq!(
        Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2),
        Solution::find_kth_largest_1(vec![3, 2, 1, 5, 6, 4], 2)
    );
    assert_eq!(
        Solution::find_kth_largest(vec![7, 6, 5, 4, 3, 2, 1], 2),
        Solution::find_kth_largest_1(vec![7, 6, 5, 4, 3, 2, 1], 2)
    );
}

struct Solution;

impl Solution {
    fn find_kth_largest_1(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        if n <= 5 {
            nums.sort();
            nums[n - k as usize]
        } else {
            Self::bfprt(&mut nums, 0, n - 1, 5, k)
        }
    }

    fn bfprt(nums: &mut [i32], l: usize, r: usize, c: usize, k: i32) -> i32 {
        let cmp_val = Self::pivot(&nums[l..=r], c);
        let mut a = l;
        let mut i = l;
        let mut xi = l;
        while i <= r {
            match nums[i].cmp(&cmp_val) {
                Ordering::Less => {
                    nums.swap(a, i);
                    a += 1
                }
                Ordering::Equal => {
                    nums.swap(a, i);
                    xi = a;
                    a += 1;
                }
                Ordering::Greater => (),
            }
            i += 1
        }
        nums.swap(xi, a - 1);
        match (nums.len() - a + 1).cmp(&(k as usize)) {
            Ordering::Equal => nums[a - 1],
            Ordering::Less => Self::bfprt(nums, l, a - 2, c, k),
            Ordering::Greater => Self::bfprt(nums, a, r, c, k),
        }
    }

    fn pivot(nums: &[i32], c: usize) -> i32 {
        let mut midst = Vec::with_capacity((nums.len() + c - 1) / c);
        let n = nums.len();
        let mut i = 0;
        while i < n {
            midst.push(Self::median(&nums[i..(i + c).min(n)]));
            i += c;
        }
        Self::median(&midst)
    }

    fn median(nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut aid = vec![0; n];
        let mut i = 0;
        while i < n {
            aid[i] = nums[i];
            let mut ti = i;
            while ti > 0 && nums[ti] < nums[ti - 1] {
                aid.swap(ti, ti - 1);
                ti -= 1;
            }

            i += 1;
        }

        aid[n >> 1]
    }

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
