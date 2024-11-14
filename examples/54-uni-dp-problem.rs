use std::mem;

fn main() {
    // assert_eq!(
    //     Solution::hard_num_decodings(String::from("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*")),
    //     Solution::hard_num_decodings_3(String::from("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*"))
    // );
    assert_eq!(
        Solution::hard_num_decodings(String::from("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*")),
        Solution::hard_num_decodings_4(String::from("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*"))
    );
}

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 1,
            1 => 1,
            _ => Self::fib(n - 1) + Self::fib(n - 2),
        }
    }

    pub fn fib1(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![-1; n + 1];
        Self::sub_fib(n, &mut dp)
    }

    fn sub_fib(n: usize, dp: &mut [i32]) -> i32 {
        match n {
            0 => {
                dp[n] = 0;
                0
            }
            1 => {
                dp[n] = 1;
                1
            }
            _ => {
                if dp[n] != -1 {
                    dp[n]
                } else {
                    dp[n] = Self::sub_fib(n - 1, dp) + Self::sub_fib(n - 2, dp);
                    dp[n]
                }
            }
        }
    }

    fn fib2(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 2];
        dp[1] = 1;
        for i in 2..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }

    fn fib3(n: i32) -> i32 {
        let mut ll = 0;
        let mut l = 1;
        match n {
            0 => ll,
            1 => l,
            _ => {
                for i in 2..=n {
                    mem::swap(&mut l, &mut ll);
                    l += ll;
                }
                l
            }
        }
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // Self::mincost_tickets_1(&days, &costs, 0);
        let mut dp = vec![-1; days.len()];
        Self::mincost_tickets_2(&days, &costs, 0, &mut dp)
    }

    fn mincost_tickets_1(days: &[i32], costs: &[i32], i: usize) -> i32 {
        if i == days.len() {
            return 0;
        }
        let mut res = i32::MAX;
        for (d, cost) in costs.iter().enumerate() {
            let mut j = i;
            res = res.min(match d {
                0 => {
                    while j < days.len() && days[i] + 1 > days[j] {
                        j += 1
                    }
                    *cost + Self::mincost_tickets_1(days, costs, j)
                }
                1 => {
                    while j < days.len() && days[i] + 7 > days[j] {
                        j += 1
                    }
                    *cost + Self::mincost_tickets_1(days, costs, j)
                }
                2 => {
                    while j < days.len() && days[i] + 30 > days[j] {
                        j += 1
                    }
                    *cost + Self::mincost_tickets_1(days, costs, j)
                }
                _ => res,
            })
        }
        res
    }

    pub fn mincost_tickets_2(days: &[i32], costs: &[i32], i: usize, dp: &mut [i32]) -> i32 {
        if i == days.len() {
            0
        } else if dp[i] != -1 {
            dp[i]
        } else {
            let mut res = i32::MAX;
            for (d, cost) in costs.iter().enumerate() {
                let mut j = i;
                res = res.min(match d {
                    0 => {
                        while j < days.len() && days[i] + 1 > days[j] {
                            j += 1
                        }
                        *cost + Self::mincost_tickets_2(days, costs, j, dp)
                    }
                    1 => {
                        while j < days.len() && days[i] + 7 > days[j] {
                            j += 1
                        }
                        *cost + Self::mincost_tickets_2(days, costs, j, dp)
                    }
                    2 => {
                        while j < days.len() && days[i] + 30 > days[j] {
                            j += 1
                        }
                        *cost + Self::mincost_tickets_2(days, costs, j, dp)
                    }
                    _ => res,
                })
            }
            dp[i] = res;
            res
        }
    }

    pub fn mincost_tickets_3(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days.len();
        let pass_days = [1, 7, 30];
        let mut dp = vec![i32::MAX; n + 1];
        dp[n] = 0;
        let mut i = n - 1;
        loop {
            for (d, cost) in costs.iter().enumerate() {
                let mut j = i;
                while j < n && days[i] + pass_days[d] > days[j] {
                    j += 1;
                }
                dp[i] = dp[i].min(*cost + dp[j])
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }
        dp[0]
    }

    pub fn num_decodings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        // Self::num_decodings_1(&s, 0)
        let mut dp = vec![-1; s.len()];
        Self::num_decodings_2(&s, 0, &mut dp)
    }

    fn num_decodings_1(s: &[char], i: usize) -> i32 {
        if i == s.len() {
            1
        } else if s[i] == '0' {
            0
        } else {
            let mut res = Self::num_decodings_1(s, i + 1);
            if i + 1 < s.len() && (s[i] as u8 - b'0') * 10 + (s[i + 1] as u8 - b'0') <= 26 {
                res += Self::num_decodings_1(s, i + 2);
            }
            res
        }
    }

    fn num_decodings_2(s: &[char], i: usize, dp: &mut [i32]) -> i32 {
        if i == s.len() {
            1
        } else if s[i] == '0' {
            0
        } else if dp[i] != -1 {
            dp[i]
        } else {
            dp[i] = Self::num_decodings_2(s, i + 1, dp);
            if i + 1 < s.len() && (s[i] as u8 - b'0') * 10 + (s[i + 1] as u8 - b'0') <= 26 {
                dp[i] += Self::num_decodings_2(s, i + 2, dp);
            }
            dp[i]
        }
    }

    pub fn num_decodings_3(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut dp = vec![0; n + 2];
        dp[n] = 1;
        for i in (0..s.len()).rev() {
            if s[i] == '0' {
                dp[i] = 0;
            } else {
                dp[i] = dp[i + 1];
                if i + 1 < s.len() && (s[i] as u8 - b'0') * 10 + (s[i + 1] as u8 - b'0') <= 26 {
                    dp[i] += dp[i + 2];
                }
            }
        }
        dp[0]
    }

    pub fn num_decodings_4(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut next_next = 0;
        let mut next = 1;
        for i in (0..n).rev() {
            let cur = if s[i] == '0' {
                0
            } else {
                let mut cur = next;
                if i + 1 < s.len() && (s[i] as u8 - b'0') * 10 + (s[i + 1] as u8 - b'0') <= 26 {
                    cur += next_next;
                }
                cur
            };
            next_next = next;
            next = cur;
        }
        next
    }

    pub fn hard_num_decodings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        // Self::hard_num_decodings_1(&s, 0)
        let mut dp = vec![-1; s.len()];
        Self::hard_num_decodings_2(&s, 0, &mut dp)
    }

    fn hard_num_decodings_1(s: &[char], i: usize) -> i32 {
        if i == s.len() {
            1
        } else if s[i] == '0' {
            0
        } else {
            let mut res = 0;
            if s[i] == '*' {
                res += 9 * Self::hard_num_decodings_1(s, i + 1);
                if i + 1 < s.len() {
                    if s[i + 1] == '*' {
                        res += 15 * Self::hard_num_decodings_1(s, i + 2);
                    } else if (s[i + 1] as u8 - b'0') <= 6 {
                        res += 2 * Self::hard_num_decodings_1(s, i + 2);
                    } else {
                        res += Self::hard_num_decodings_1(s, i + 2)
                    }
                }
            } else {
                res += Self::hard_num_decodings_1(s, i + 1);
                if i + 1 < s.len() {
                    if s[i + 1] == '*' {
                        if s[i] == '1' {
                            res += 9 * Self::hard_num_decodings_1(s, i + 2);
                        } else if s[i] == '2' {
                            res += 6 * Self::hard_num_decodings_1(s, i + 2);
                        }
                    } else if (s[i] as u8 - b'0') * 10 + s[i + 1] as u8 - b'0' <= 26 {
                        res += Self::hard_num_decodings_1(s, i + 2);
                    }
                }
            }
            res
        }
    }

    const MOD: i64 = 1_000_000_007;

    fn hard_num_decodings_2(s: &[char], i: usize, dp: &mut [i64]) -> i32 {
        if i == s.len() {
            1
        } else if s[i] == '0' {
            0
        } else if dp[i] != -1 {
            dp[i] as i32
        } else {
            let mut res: i64 = 0;
            if s[i] == '*' {
                res += 9 * Self::hard_num_decodings_2(s, i + 1, dp) as i64;
                if i + 1 < s.len() {
                    if s[i + 1] == '*' {
                        res += 15 * Self::hard_num_decodings_2(s, i + 2, dp) as i64;
                    } else if (s[i + 1] as u8 - b'0') <= 6 {
                        res += 2 * Self::hard_num_decodings_2(s, i + 2, dp) as i64;
                    } else {
                        res += Self::hard_num_decodings_2(s, i + 2, dp) as i64
                    }
                }
            } else {
                res += Self::hard_num_decodings_2(s, i + 1, dp) as i64;
                if i + 1 < s.len() {
                    if s[i + 1] == '*' {
                        if s[i] == '1' {
                            res += 9 * Self::hard_num_decodings_2(s, i + 2, dp) as i64;
                        } else if s[i] == '2' {
                            res += 6 * Self::hard_num_decodings_2(s, i + 2, dp) as i64;
                        }
                    } else if (s[i] as u8 - b'0') * 10 + s[i + 1] as u8 - b'0' <= 26 {
                        res += Self::hard_num_decodings_2(s, i + 2, dp) as i64;
                    }
                }
            }
            dp[i] = res % Self::MOD;
            dp[i] as i32
        }
    }

    fn hard_num_decodings_3(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut dp = vec![0_i64; n + 2];
        dp[n] = 1;
        for i in (0..n).rev() {
            if s[i] == '0' {
                dp[i] = 0;
            } else if s[i] == '*' {
                dp[i] += 9 * dp[i + 1];
                if i + 1 < n {
                    if s[i + 1] == '*' {
                        dp[i] += 15 * dp[i + 2];
                    } else if (s[i + 1] as u8 - b'0') <= 6 {
                        dp[i] += 2 * dp[i + 2];
                    } else {
                        dp[i] += dp[i + 2];
                    }
                }
            } else {
                dp[i] += dp[i + 1];
                if i + 1 < n {
                    if s[i + 1] == '*' {
                        if s[i] == '1' {
                            dp[i] += 9 * dp[i + 2];
                        } else if s[i] == '2' {
                            dp[i] += 6 * dp[i + 2];
                        }
                    } else if (s[i] as u8 - b'0') * 10 + s[i + 1] as u8 - b'0' <= 26 {
                        dp[i] += dp[i + 2];
                    }
                }
            }
            dp[i] %= Self::MOD;
        }

        dp[0] as i32
    }

    fn hard_num_decodings_4(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut next_next = 0;
        let mut next = 1;
        for i in (0..n).rev() {
            let mut cur = 0;
            if s[i] != '0' {
                if s[i] == '*' {
                    cur += 9 * next;
                    if i + 1 < n {
                        if s[i + 1] == '*' {
                            cur += 15 * next_next;
                        } else if (s[i + 1] as u8 - b'0') <= 6 {
                            cur += 2 * next_next;
                        } else {
                            cur += next_next;
                        }
                    }
                } else {
                    cur += next;
                    if i + 1 < n {
                        if s[i + 1] == '*' {
                            if s[i] == '1' {
                                cur += 9 * next_next;
                            } else if s[i] == '2' {
                                cur += 6 * next_next;
                            }
                        } else if (s[i] as u8 - b'0') * 10 + s[i + 1] as u8 - b'0' <= 26 {
                            cur += next_next;
                        }
                    }
                }
            }
            cur %= Self::MOD;
            next_next = next;
            next = cur;
        }
        next as i32
    }
}
