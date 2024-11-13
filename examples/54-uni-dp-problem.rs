use std::mem;

fn main() {}

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
}
