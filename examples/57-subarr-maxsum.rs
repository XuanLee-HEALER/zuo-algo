use std::i32;

fn main() {
    println!(
        "res: {}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!(
        "extra res: {:?}",
        Solution::max_sub_array_extra(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!(
        "extra res: {:?}",
        Solution::max_sub_array_extra(vec![5, 4, -1, 7, 8])
    );
    println!(
        "circular res: {}",
        Solution::max_subarray_sum_circular(vec![1, -2, 3, -2])
    );
    println!(
        "max submatrix: {:?}",
        Solution::get_max_matrix(vec![
            vec![9, -8, 1, 3, -2],
            vec![-3, 7, 6, -2, 4],
            vec![6, -4, -4, 8, -7]
        ])
    )
}

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = nums[0];
        let mut ans = pre;
        for &num in &nums[1..] {
            pre = num.max(num + pre);
            ans = ans.max(pre)
        }
        ans
    }

    fn max_sub_array_extra(nums: Vec<i32>) -> (usize, usize, i32) {
        let mut r = (0, 0, i32::MIN);
        let (mut tl, mut tr, mut sum) = (0, 0, i32::MIN);
        while tr < nums.len() {
            if sum < 0 {
                tl = tr;
                sum = nums[tr];
            } else {
                sum += nums[tr];
            }
            if sum > r.2 {
                r.0 = tl;
                r.1 = tr;
                r.2 = sum;
            }
            tr += 1;
        }
        r
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            _ => {
                let mut prepre = nums[0];
                let mut pre = prepre.max(nums[1]);
                for &num in &nums[2..] {
                    let cur = num.max(num + prepre).max(pre);
                    prepre = pre;
                    pre = cur;
                }
                pre
            }
        }
    }

    // 题目要求数组内元素全部大于等于0，所以不存在只选当前值不选的情况
    fn rob_1(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |acc, e| (acc.1, acc.1.max(e + acc.0)))
            .1
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut all = nums[0];
        let mut max = nums[0];
        let mut min = nums[0];
        let mut max_pre = nums[0];
        let mut min_pre = nums[0];
        for &num in &nums[1..] {
            all += num;
            max_pre = num.max(num + max_pre);
            min_pre = num.min(num + min_pre);
            max = max.max(max_pre);
            min = min.min(min_pre);
        }
        if all == min {
            max
        } else {
            max.max(all - min)
        }
    }

    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            nums[0]
        } else if n == 2 {
            nums[0].max(nums[1])
        } else {
            Self::rob_2_1(&nums[1..]).max(nums[0] + Self::rob_2_1(&nums[2..n - 1]))
        }
    }

    fn rob_2_1(nums: &[i32]) -> i32 {
        let n = nums.len();
        if n == 0 {
            0
        } else if n == 1 {
            nums[0]
        } else if n == 2 {
            nums[0].max(nums[1])
        } else {
            let mut prepre = nums[0];
            let mut pre = nums[0].max(nums[1]);
            for &num in &nums[2..] {
                let cur = pre.max(num + prepre).max(num);
                prepre = pre;
                pre = cur;
            }
            pre
        }
    }

    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        nums.iter().for_each(|&num| {
            min = min.min(num);
            max = max.max(num);
        });
        let mut res = 0;
        while min <= max {
            let mid = min + ((max - min) >> 1);
            if Self::mc(&nums, mid) >= k {
                res = mid;
                if mid > min {
                    max = mid;
                } else {
                    break;
                }
            } else {
                min = mid + 1;
            }
        }
        res
    }

    fn mc(nums: &[i32], cap: i32) -> i32 {
        match nums.len() {
            1 => {
                if nums[0] <= cap {
                    1
                } else {
                    0
                }
            }
            2 => {
                if nums[0] <= cap || nums[1] <= cap {
                    1
                } else {
                    0
                }
            }
            _ => {
                let mut prepre = if nums[0] <= cap { 1 } else { 0 };
                let mut pre = if nums[0] <= cap || nums[1] <= cap {
                    1
                } else {
                    0
                };
                for &num in &nums[2..] {
                    let cur = pre.max(if num <= cap { 1 + prepre } else { 0 });
                    prepre = pre;
                    pre = cur
                }
                pre
            }
        }
    }

    fn mc_1(nums: &[i32], cap: i32) -> i32 {
        let mut res = 0;
        let mut skip = false;
        for &num in nums {
            if !skip {
                if num <= cap {
                    res += 1;
                    skip = true;
                }
            } else {
                skip = false
            }
        }
        res
    }

    pub fn get_max_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut aid = vec![0; m];
        let sum_to_aid = |aid: &mut [i32], nums: &[i32]| {
            aid.iter_mut().enumerate().for_each(|(i, e)| *e += nums[i]);
        };
        let mut max = i32::MIN;
        let mut res = vec![0, 0, 0, 0];
        for i in 0..n {
            for (ti, sub) in matrix.iter().skip(i).enumerate() {
                sum_to_aid(&mut aid, sub);
                let mut pre = i32::MIN;
                let mut cur_max = pre;
                let mut l = 0;
                let mut r = 0;
                let mut max_l = 0;
                let mut max_r = 0;
                while r < m {
                    if pre <= 0 {
                        pre = aid[r];
                        l = r;
                    } else {
                        pre += aid[r];
                    }
                    if pre > cur_max {
                        cur_max = pre;
                        max_l = l;
                        max_r = r;
                    }
                    r += 1;
                }
                if cur_max > max {
                    max = cur_max;
                    res[0] = i as i32;
                    res[1] = max_l as i32;
                    res[2] = (i + ti) as i32;
                    res[3] = max_r as i32;
                }
            }
            aid.fill(0);
        }
        res
    }
}
