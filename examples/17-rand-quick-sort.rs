use std::cmp::Ordering;

use rand::{seq::SliceRandom, Rng};

fn main() {
    let mut rand_arr = (1..=30).collect::<Vec<i32>>();
    rand_arr.shuffle(&mut rand::thread_rng());
    println!("ori: {rand_arr:?}");
    let res = Solution::sort_array(rand_arr);
    println!("res: {res:?}");
}

struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let arr_len = nums.len();
        Self::quick_sort(&mut nums, 0, arr_len - 1, &mut rng);
        nums
    }

    fn quick_sort<R: Rng + ?Sized>(nums: &mut [i32], l: usize, r: usize, rng: &mut R) {
        if l < r {
            let ri = rng.gen_range(l..=r);
            // let ri = Self::partition1(nums, l, r, ri);
            // if ri > 0 {
            //     Self::quick_sort(nums, l, ri - 1, rng);
            // }
            // Self::quick_sort(nums, ri + 1, r, rng);
            let (tl, tr) = Self::partition2(nums, l, r, ri);
            if tl > 0 {
                Self::quick_sort(nums, l, tl - 1, rng);
            }
            Self::quick_sort(nums, tr + 1, r, rng);
        }
    }

    fn partition1(nums: &mut [i32], l: usize, r: usize, mid: usize) -> usize {
        // 由于没有额外的数组空间，所以这里要记录比较的值，因为数组会变，也可以直接将值传进来而不是索引
        let cmp_val = nums[mid];
        let mut a = l;
        let mut i = l;
        let mut xi = l;
        while i <= r {
            if nums[i] <= cmp_val {
                if nums[i] == cmp_val {
                    // xi需要记录的是等于这个数的任意一个位置，而不是原位置
                    xi = a;
                }
                nums.swap(i, a);
                a += 1;
            }
            i += 1;
        }
        nums.swap(xi, a - 1);
        a - 1
    }

    fn partition2(nums: &mut [i32], l: usize, r: usize, mid: usize) -> (usize, usize) {
        let mut a = l;
        let mut b = r;
        let mut i = l;
        let cmp_val = nums[mid];
        while i <= b {
            match nums[i].cmp(&cmp_val) {
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
