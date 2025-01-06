fn main() {
    println!(
        "res {}",
        Solution::can_partition_k_subsets_1(vec![4, 3, 2, 3, 5, 2, 1], 4)
    )
}

struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total <= 0 {
            true
        } else if ((1 + max_choosable_integer) / 2 * max_choosable_integer) < desired_total {
            false
        } else {
            let status = (1 << (max_choosable_integer + 1)) - 1;
            let mut dp = vec![0; status as usize + 1];
            Self::ciw(status, max_choosable_integer, desired_total, &mut dp)
        }
    }

    fn ciw(status: i32, n: i32, remain: i32, dp: &mut [i32]) -> bool {
        if remain <= 0 {
            false
        } else if dp[status as usize] != 0 {
            if dp[status as usize] == 1 {
                true
            } else {
                false
            }
        } else {
            let mut ans = false;
            for c in 1..=n {
                if status & (1 << c) > 0 && !Self::ciw(status ^ (1 << c), n, remain - c, dp) {
                    ans = true;
                    break;
                }
            }
            dp[status as usize] = if ans { 1 } else { -1 };
            ans
        }
    }

    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            false
        } else {
            let limit = sum / 4;
            let n = matchsticks.len();
            let status = (1 << n) - 1;
            let mut dp = vec![0; status as usize + 1];
            Self::ms(&matchsticks, status, limit, 0, 4, &mut dp)
        }
    }

    fn ms(
        matchsticks: &[i32],
        status: i32,
        limit: i32,
        cur: i32,
        remain: i32,
        dp: &mut [i32],
    ) -> bool {
        if status == 0 && remain == 0 {
            true
        } else if dp[status as usize] != 0 {
            if dp[status as usize] == 1 {
                true
            } else {
                false
            }
        } else {
            let mut ans = false;
            for (i, &stick) in matchsticks.iter().enumerate() {
                if (1 << i) & status > 0 && cur + stick <= limit {
                    ans = if cur + stick < limit {
                        Self::ms(
                            matchsticks,
                            status ^ (1 << i),
                            limit,
                            cur + stick,
                            remain,
                            dp,
                        )
                    } else {
                        Self::ms(matchsticks, status ^ (1 << i), limit, 0, remain - 1, dp)
                    };
                    if ans {
                        break;
                    }
                }
            }
            dp[status as usize] = if ans { 1 } else { -1 };
            ans
        }
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            false
        } else {
            let limit = sum / k;
            let n = nums.len();
            let status = (1 << n) - 1;
            let mut dp = vec![0; status as usize + 1];
            Self::ms(&nums, status, limit, 0, k, &mut dp)
        }
    }

    pub fn can_partition_k_subsets_1(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            false
        } else {
            let mut nums = nums;
            nums.sort();
            let mut res = vec![0; k as usize];
            Self::cpks(&mut res, &nums, sum / k, nums.len() as i32 - 1)
        }
    }

    fn cpks(res: &mut [i32], nums: &[i32], limit: i32, idx: i32) -> bool {
        if idx < 0 {
            true
        } else {
            let mut i = 0;
            while i < res.len() {
                let cur = nums[idx as usize];
                if res[i] + cur <= limit {
                    res[i] += cur;
                    if Self::cpks(res, nums, limit, idx - 1) {
                        return true;
                    }
                    res[i] -= cur;
                }

                if i + 1 < res.len() && res[i + 1] == res[i] {
                    i += 1;
                }
                i += 1;
            }
            false
        }
    }
}
