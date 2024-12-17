fn main() {
    println!("res: {}", Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]))
}

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // Self::dumb_increasing_seq(&nums)
        Self::increasing_seq(&nums)
    }

    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut env = envelopes;
        env.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => b[1].cmp(&a[1]),
            _ => a[0].cmp(&b[0]),
        });
        let heights = env.iter().map(|v| v[1]).collect::<Vec<i32>>();
        Self::increasing_seq(&heights)
    }

    fn increasing_seq(nums: &[i32]) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut aid_len = 0;
        let mut aid = vec![0; n];
        aid[0] = nums[0];
        aid_len += 1;
        for &num in nums.iter().skip(1) {
            if let Some(i) = Self::bs(&aid, aid_len, num) {
                aid[i] = num;
            } else {
                aid[aid_len] = num;
                aid_len += 1;
            }
        }
        aid_len as i32
    }

    fn bs(arr: &[i32], len: usize, val: i32) -> Option<usize> {
        let (mut l, mut r) = (0, len - 1);
        let mut res = None;
        while l <= r {
            let m = l + ((r - l) >> 1);
            if arr[m] >= val {
                res = Some(m);
                if m > 0 {
                    r = m - 1
                } else {
                    break;
                }
            } else {
                l = m + 1;
            }
        }
        res
    }

    fn dumb_increasing_seq(nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut max = dp[0];
        for (i, &num) in nums.iter().skip(1).enumerate() {
            for j in (0..i + 1).rev() {
                if num > nums[j] {
                    dp[i + 1] = dp[i + 1].max(dp[j] + 1)
                }
            }
            max = max.max(dp[i + 1])
        }
        max
    }

    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let k = k as usize;
        let mut grp = vec![0; n.div_ceil(k)];
        let mut res = 0;
        for i in 0..k {
            let mut c = 0;
            grp[c] = arr[i];
            c += 1;
            let mut j = i + k;
            while j < n {
                grp[c] = arr[j];
                j += k;
                c += 1;
            }
            let t = Self::longest_not_decrease(&grp[..c]);
            res += c as i32 - t;
        }
        res
    }

    fn longest_not_decrease(nums: &[i32]) -> i32 {
        let n = nums.len();
        if n == 0 {
            0
        } else {
            let mut aid = vec![nums[0]; nums.len()];
            let mut l = 1;
            for &num in nums.iter().skip(1) {
                if let Some(idx) = Self::bs2(&aid, l, num) {
                    aid[idx] = aid[idx].min(num);
                } else {
                    aid[l] = num;
                    l += 1;
                }
            }

            l as i32
        }
    }

    fn bs2(nums: &[i32], len: usize, val: i32) -> Option<usize> {
        let (mut l, mut r) = (0, len - 1);
        let mut res = None;
        while l <= r {
            let m = l + ((r - l) >> 1);
            if nums[m] > val {
                res = Some(m);
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

    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_unstable_by_key(|v| v[0]);
        let mut aid = vec![pairs[0][1]; pairs.len()];
        let mut l = 1;
        for pair in pairs.iter().skip(1) {
            let (left, right) = (pair[0], pair[1]);
            if let Some(idx) = Self::bs(&aid, l, left) {
                aid[idx] = aid[idx].min(right);
            } else {
                aid[l] = right;
                l += 1;
            }
        }
        l as i32
    }

    pub fn find_longest_chain_1(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_unstable_by_key(|v| v[1]);
        let mut cmp = pairs[0][1];
        let mut res = 1;
        for pair in pairs.iter().skip(1) {
            if pair[0] > cmp {
                res += 1;
                cmp = pair[1];
            }
        }
        res
    }
}
