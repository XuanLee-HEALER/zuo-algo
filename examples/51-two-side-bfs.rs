use std::{collections::HashSet, i32, mem};

fn main() {
    println!(
        "{:?}",
        Solution::ladder_length(
            "hit".into(),
            "cog".into(),
            vec![
                "hot".into(),
                "dot".into(),
                "dog".into(),
                "lot".into(),
                "log".into(),
                "cog".into()
            ]
        )
    );
    println!("ans {}", Solution::min_abs_difference(vec![5, -7, 3, 5], 6));
}

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut base = word_list.iter().collect::<HashSet<&String>>();
        if !base.contains(&end_word) {
            return 0;
        }
        let p_words = |word: &str| -> Vec<String> {
            let mut ans = Vec::new();
            let mut cs = word.chars().collect::<Vec<char>>();
            for (i, c) in word.char_indices() {
                let old_c = c;
                for j in 0..=25_u8 {
                    let rc = (b'a' + j) as char;
                    if rc == old_c {
                        continue;
                    }
                    cs[i] = rc;
                    ans.push(cs.iter().collect::<String>());
                }
                cs[i] = old_c;
            }
            ans
        };
        let mut small = HashSet::new();
        let mut big = HashSet::new();
        let mut next = HashSet::new();
        small.insert(begin_word);
        big.insert(end_word);
        let mut ans = 2;

        while !small.is_empty() {
            for small_str in &small {
                for n_str in p_words(small_str) {
                    if big.contains(&n_str) {
                        return ans;
                    } else if base.contains(&n_str) {
                        next.insert(n_str);
                    }
                }
            }

            base.retain(|&e| !next.contains(e));

            // 如果下一轮更多，那么下一轮从big开始，big和small交换
            // 如果big和small交换，next需要替换big
            if next.len() > big.len() {
                mem::swap(&mut big, &mut small);
                mem::swap(&mut big, &mut next);
            } else {
                // 如果下一轮更少，那么small和next交换
                mem::swap(&mut small, &mut next);
            }
            next.clear();
            ans += 1;
        }

        0
    }

    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let t_max = nums.iter().filter(|&e| *e >= 0).sum();
        let t_min = nums.iter().filter(|&e| *e < 0).sum();
        if goal >= t_max {
            return goal - t_max;
        }
        if goal <= t_min {
            return t_min - goal;
        }
        let mut left_group = vec![0; 1 << ((n + 1) >> 1)];
        let mut right_group = vec![0; 1 << ((n + 1) >> 1)];
        nums.sort_unstable();
        let ll = Self::collect(&mut left_group, &nums, 0, 0, n >> 1, 0);
        let rl = Self::collect(&mut right_group, &nums, 0, n >> 1, n, 0);
        let left_group = &mut left_group[..ll];
        let right_group = &mut right_group[..rl];
        left_group.sort_unstable();
        right_group.sort_unstable();

        let mut ans = goal.abs() as i64;
        let mut i = 0;
        let mut j = rl - 1;
        while i < ll {
            while j > 0
                && (left_group[i] + right_group[j - 1] - goal as i64).abs()
                    <= (left_group[i] + right_group[j] - goal as i64).abs()
            {
                j -= 1;
            }
            ans = ans.min((left_group[i] + right_group[j] - goal as i64).abs());

            i += 1;
        }

        ans as i32
    }

    fn collect(
        group: &mut [i64],
        nums: &[i32],
        sum: i64,
        begin: usize,
        end: usize,
        mut last: usize,
    ) -> usize {
        if begin == end {
            group[last] = sum;
            last += 1;
        } else {
            let mut j = begin + 1;
            while j < end && nums[j] == nums[begin] {
                j += 1;
            }
            for k in 0..=(j - begin) {
                last = Self::collect(
                    group,
                    nums,
                    (k as i32 * nums[begin]) as i64 + sum,
                    j,
                    end,
                    last,
                );
            }
        }
        last
    }
}
