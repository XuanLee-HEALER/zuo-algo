fn main() {
    println!("{}", Solution::count_numbers_with_unique_digits(3));
    println!(
        "{}",
        Solution::at_most_n_given_digit_set(vec!["1".into(), "4".into(), "9".into()], 1000000000)
    );
    println!(
        "{}",
        Solution::at_most_n_given_digit_set_1(vec!["1".into(), "4".into(), "9".into()], 1000000000)
    );
    println!("{}", Solution::count("1".into(), "12".into(), 1, 8));
    println!("{}", Solution::count_special_numbers(135));
    // println!("{}", Solution::count_special_numbers(43));
}

struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            let mut res = 10;
            let mut ini = 9;
            let mut i = 9;
            for _ in 2..=n {
                ini *= i;
                res += ini;
                i -= 1;
            }
            res
        }
    }

    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let mut nn = n / 10;
        let mut len = 1;
        let mut offset = 1;
        while nn > 0 {
            len += 1;
            offset *= 10;
            nn /= 10;
        }
        let digits = digits
            .iter()
            .map(|s| s.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Self::amngds(&digits, n, len, offset, false, true)
    }

    fn amngds(digits: &[i32], num: i32, len: i32, offset: i32, free: bool, skip: bool) -> i32 {
        if len == 0 {
            if skip {
                0
            } else {
                1
            }
        } else {
            let mut res = 0;
            let cur = (num / offset) % 10;
            if skip {
                res += Self::amngds(digits, num, len - 1, offset / 10, true, true)
            }
            if free {
                res += digits.len() as i32
                    * Self::amngds(digits, num, len - 1, offset / 10, true, false)
            } else {
                for &d in digits {
                    if d < cur {
                        res += Self::amngds(digits, num, len - 1, offset / 10, true, false)
                    } else if d == cur {
                        res += Self::amngds(digits, num, len - 1, offset / 10, false, false)
                    } else {
                        break;
                    }
                }
            }
            res
        }
    }

    pub fn at_most_n_given_digit_set_1(digits: Vec<String>, n: i32) -> i32 {
        let mut nn = n / 10;
        let mut len = 1;
        let mut offset = 1;
        while nn > 0 {
            len += 1;
            offset *= 10;
            nn /= 10;
        }
        let digits = digits
            .iter()
            .map(|s| s.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut res = 0;
        let t = digits.len() as i32;
        let mut m = t;
        // 在i位填小于目标数的数时，会有多少种可能
        // prep[1] = 1，表示如果1位上填了更小的数字，那么只有自己的一种可能性
        let mut prep = vec![1; len as usize + 1];
        for i in 2..=len as usize {
            prep[i] *= m;
            res += prep[i];
            m *= t;
        }
        res + Self::amngds_1(&digits, &prep, n, len, offset)
    }

    pub fn amngds_1(digits: &[i32], prep: &[i32], num: i32, len: i32, offset: i32) -> i32 {
        if len == 0 {
            1
        } else {
            let mut res = 0;
            let cur = (num / offset) % 10;
            for &d in digits {
                if d < cur {
                    res += prep[len as usize]
                } else if d == cur {
                    res += Self::amngds_1(digits, prep, num, len - 1, offset / 10)
                } else {
                    break;
                }
            }
            res
        }
    }

    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let mut dp = vec![vec![vec![-1; 2]; 400]; 23];
        let r1 = Self::c(num1, min_sum, max_sum, 0, 0, 0, &mut dp);
        for sub in dp.iter_mut() {
            for sub in sub.iter_mut() {
                for v in sub.iter_mut() {
                    *v = -1
                }
            }
        }
        let mut r2 = Self::c(num2, min_sum, max_sum, 0, 0, 0, &mut dp);
        r2 = (r2 - r1 + Self::MOD) % Self::MOD;
        let nr = num1
            .iter()
            .map(|s| (*s - b'0') as i32)
            .reduce(|a, b| a + b)
            .unwrap();
        if nr >= min_sum && nr <= max_sum {
            r2 + 1
        } else {
            r2
        }
    }

    fn c(
        nums: &[u8],
        min_sum: i32,
        max_sum: i32,
        i: i32,
        presum: i32,
        free: i32,
        dp: &mut [Vec<Vec<i32>>],
    ) -> i32 {
        if presum > max_sum || presum + (nums.len() as i32 - i) * 9 < min_sum {
            0
        } else if i as usize == nums.len() {
            1
        } else if dp[i as usize][presum as usize][free as usize] != -1 {
            dp[i as usize][presum as usize][free as usize]
        } else {
            let mut res = 0;
            if free == 0 {
                let cur = (nums[i as usize] - b'0') as i32;
                for j in 0..cur {
                    res = (res + Self::c(nums, min_sum, max_sum, i + 1, presum + j, 1, dp))
                        % Self::MOD
                }
                res =
                    (res + Self::c(nums, min_sum, max_sum, i + 1, presum + cur, 0, dp)) % Self::MOD
            } else {
                for j in 0..10 {
                    res = (res + Self::c(nums, min_sum, max_sum, i + 1, presum + j, 1, dp))
                        % Self::MOD;
                }
            }
            dp[i as usize][presum as usize][free as usize] = res;
            res
        }
    }

    const MOD: i32 = 1_000_000_007;

    pub fn count_special_numbers(n: i32) -> i32 {
        let mut nn = n / 10;
        let mut len: i32 = 1;
        let mut offset = 1;
        while nn > 0 {
            len += 1;
            offset *= 10;
            nn /= 10;
        }
        let mut precnt = vec![1; len as usize];
        let mut cnt = 10 - len + 1;
        let mut t = 1;
        for i in 1..len {
            t *= cnt;
            precnt[i as usize] = t;
            cnt += 1;
        }
        let mut res = 0;
        let mut cnt = 9;
        let mut t = 1;
        for i in 0..len - 1 {
            t *= cnt;
            if i >= 1 {
                cnt -= 1;
            }
            res += t;
        }
        let f = (n / offset) % 10;
        res += (f - 1) * precnt[len as usize - 1];
        res += Self::csn(n, &precnt, len - 1, offset / 10, 1 << f);

        res
    }

    fn csn(num: i32, precnt: &[i32], len: i32, offset: i32, status: i32) -> i32 {
        if len == 0 {
            1
        } else {
            let mut res = 0;
            let cur = (num / offset) % 10;
            for i in 0..cur {
                if (1 << i) & status == 0 {
                    res += precnt[len as usize - 1]
                }
            }
            if (1 << cur) & status == 0 {
                res += Self::csn(num, precnt, len - 1, offset / 10, status | (1 << cur))
            }
            res
        }
    }

    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        n - Self::count_special_numbers(n)
    }
}
