use std::{collections::HashMap, i32};

fn main() {
    println!("跟着左程云学算法!");
}

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = HashMap::new();
        counter.insert(0, 1);
        let mut sum = 0;
        let mut ans = 0;
        for num in &nums {
            sum += num;
            let aim = sum - k;
            if let Some(cnt) = counter.get(&aim) {
                ans += cnt
            }
            counter.entry(sum).and_modify(|v| *v += 1).or_insert(1);
        }

        ans
    }

    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut pre_loc = HashMap::new();
        pre_loc.insert(0, -1);

        let mut sum = 0;
        let mut ans = 0;
        for (idx, hour) in hours.iter().enumerate() {
            sum += if *hour > 8 { 1 } else { -1 };
            if sum > 0 {
                ans = ans.max(idx as i32 + 1);
            } else if let Some(v) = pre_loc.get(&(sum - 1)) {
                ans = ans.max(idx as i32 - *v);
            }

            pre_loc.entry(sum).or_insert(idx as i32);
        }

        ans
    }

    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut rear_loc = HashMap::new();
        rear_loc.insert(0, -1);
        let mut rem = 0;
        for num in &nums {
            rem = (rem + num) % p;
        }

        if rem == 0 {
            return 0;
        }

        let mut cur_mod: i32 = 0;
        let mut ans = i32::MAX;
        for (idx, num) in nums.iter().enumerate() {
            cur_mod = (cur_mod + num) % p;
            let find = (p - rem + cur_mod) % p;
            if let Some(v) = rear_loc.get(&find) {
                ans = ans.min(idx as i32 - *v);
            }
            rear_loc.insert(cur_mod, idx as i32);
        }

        if ans == nums.len() as i32 {
            -1
        } else {
            ans
        }
    }

    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut pre_loc: [i32; 32] = [-2; 32];
        // 全偶情况
        pre_loc[0] = -1;

        let mut status = 0;
        let mut ans = 0;
        for (idx, c) in s.chars().enumerate() {
            let xidx = match c {
                'a' => 0,
                'e' => 1,
                'i' => 2,
                'o' => 3,
                'u' => 4,
                _ => -1,
            };
            if xidx != -1 {
                status ^= 1 << xidx;
            }
            if pre_loc[status] == -2 {
                pre_loc[status] = idx as i32
            } else {
                ans = ans.max(idx as i32 - pre_loc[status])
            }
        }

        ans
    }
}

struct NumArray {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut fnum = Vec::new();
        fnum.push(0);
        let mut sum = 0;
        for num in nums {
            sum += num;
            fnum.push(sum);
        }
        Self { nums: fnum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        // 2, 5
        // 2是 0 1 2的和，存放在 3
        // 5是 0 1 2 3 4 5，存放在 6
        // 2-5 是2 3 4 5，所以是 6位置 - 2位置
        self.nums[right as usize + 1] - self.nums[left as usize]
    }
}
