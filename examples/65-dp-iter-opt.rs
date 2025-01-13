use std::mem;

fn main() {
    println!(
        "res {}",
        Solution::max_profit_iv_2(2, vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0])
    );
    println!("res {}", Solution::max_profit_vi(vec![2, 1, 4]));
    println!("res {}", Solution::num_perms_di_sequence_2("DID".into()))
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut min = prices[0];
        for p in prices.into_iter() {
            ans = ans.max(p - min);
            min = min.min(p);
        }
        ans
    }

    pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut base = prices[0];
        for p in prices.into_iter() {
            if p > base {
                ans += p - base;
            }
            base = p;
        }
        ans
    }

    pub fn max_profit_iii_0(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp1 = vec![0; n];
        let mut t_min = prices[0];
        for i in 1..n {
            dp1[i] = (prices[i] - t_min).max(dp1[i - 1]);
            t_min = t_min.min(prices[i])
        }
        // 0~i天，以i做第二次交易卖出天数能得到的最大利润
        let mut res = 0;
        let mut dp2 = vec![0; n];
        for i in 1..n {
            for k in 0..i {
                dp2[i] = dp2[i].max(dp1[k] + prices[i] - prices[k])
            }
            res = res.max(dp2[i])
        }

        res
    }

    pub fn max_profit_iii_1(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp1 = vec![0; n];
        let mut best = vec![0; n];
        best[0] = -prices[0];
        let mut t_min = prices[0];
        let mut res = 0;
        let mut dp2 = vec![0; n];
        for i in 1..n {
            dp1[i] = (prices[i] - t_min).max(dp1[i - 1]);
            best[i] = (dp1[i] - prices[i]).max(best[i - 1]);
            t_min = t_min.min(prices[i]);
            // 0~i天，以i做第二次交易卖出天数能得到的最大利润
            dp2[i] = dp2[i].max(best[i] + prices[i]);
            res = res.max(dp2[i])
        }

        res
    }

    pub fn max_profit_iii_2(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut t_min = prices[0];
        let mut res = 0;
        let mut dp1_pre = 0;
        let mut best_pre = -prices[0];
        for i in 1..n {
            let dp1_cur = (prices[i] - t_min).max(dp1_pre);
            let best_cur = (dp1_cur - prices[i]).max(best_pre);
            t_min = t_min.min(prices[i]);
            // 0~i天，以i做第二次交易卖出天数能得到的最大利润
            res = res.max(best_cur + prices[i]);
            dp1_pre = dp1_cur;
            best_pre = best_cur;
        }

        res
    }

    pub fn max_profit_iv_0(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let n = prices.len();
        if k >= n / 2 {
            let mut res = 0;
            let mut last = prices[0];
            for &p in prices.iter().skip(1) {
                if p > last {
                    res += p - last;
                }
                last = p;
            }
            res
        } else {
            let mut dp = vec![vec![0; n]; k + 1];
            for i in 1..k + 1 {
                for j in 1..n {
                    dp[i][j] = dp[i][j - 1];
                    for k in 0..j {
                        dp[i][j] = dp[i][j].max(dp[i - 1][k] + prices[j] - prices[k])
                    }
                }
            }
            dp[k][n - 1]
        }
    }

    pub fn max_profit_unlimit(prices: &[i32]) -> i32 {
        let mut ans = 0;
        let mut base = prices[0];
        for &p in prices {
            if p > base {
                ans += p - base;
            }
            base = p;
        }
        ans
    }

    pub fn max_profit_iv_1(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let n = prices.len();
        if k >= n / 2 {
            Self::max_profit_unlimit(&prices)
        } else {
            let mut dp = vec![vec![0; n]; k + 1];
            for i in 1..k + 1 {
                let mut best = -prices[0];
                for j in 1..n {
                    dp[i][j] = dp[i][j - 1].max(best + prices[j]);
                    best = best.max(dp[i - 1][j] - prices[j]);
                }
            }
            dp[k][n - 1]
        }
    }

    pub fn max_profit_iv_2(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let n = prices.len();
        if k >= n / 2 {
            Self::max_profit_unlimit(&prices)
        } else {
            let mut dp = vec![0; n];
            for _ in 1..k + 1 {
                let mut best = -prices[0];
                for j in 1..n {
                    let tmp = dp[j];
                    dp[j] = dp[j - 1].max(best + prices[j]);
                    best = best.max(tmp - prices[j]);
                }
            }
            dp[n - 1]
        }
    }

    pub fn max_profit_v(prices: Vec<i32>, fee: i32) -> i32 {
        let mut prepare = -prices[0] - fee;
        let mut dane = 0;
        for price in prices.into_iter() {
            dane = dane.max(prepare + price);
            prepare = prepare.max(dane - price - fee);
        }
        dane
    }

    pub fn max_profit_vi(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            0
        } else {
            let mut prep1 = (-prices[0]).max(-prices[1]);
            let mut dane1 = 0;
            let mut dane2 = 0.max(prep1 + prices[1]);
            let mut i = 2;
            while i < n {
                let cur = dane2.max(prep1 + prices[i]);
                prep1 = prep1.max(dane1 - prices[i]);
                dane1 = dane2;
                dane2 = cur;
                i += 1
            }
            dane2
        }
    }

    pub fn num_perms_di_sequence(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        if n <= 1 {
            1
        } else {
            Self::npds(s, 0, n + 1, n + 1)
        }
    }

    fn npds(arr: &[u8], i: usize, less: usize, n: usize) -> i32 {
        if i == n {
            1
        } else {
            let mut res = 0;
            if i == 0 || arr[i - 1] == b'D' {
                for l in 0..less {
                    res = (res + Self::npds(arr, i + 1, l, n)) % Self::MOD
                }
            } else {
                for l in less..(n - i) {
                    res = (res + Self::npds(arr, i + 1, l, n)) % Self::MOD
                }
            }
            res
        }
    }

    pub fn num_perms_di_sequence_1(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n + 2]; n + 2];
        dp.last_mut().unwrap().fill(1);
        for i in (0..n + 1).rev() {
            for j in 0..n + 2 {
                let mut res = 0;
                if i == 0 || s[i - 1] == b'D' {
                    for k in 0..j {
                        res = (res + dp[i + 1][k]) % Self::MOD
                    }
                } else {
                    for k in j..(n + 1 - i) {
                        res = (res + dp[i + 1][k]) % Self::MOD
                    }
                }
                dp[i][j] = res
            }
        }
        dp[0][n + 1]
    }

    pub fn num_perms_di_sequence_2(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n + 2]; n + 2];
        dp.last_mut().unwrap().fill(1);
        for i in (0..n + 1).rev() {
            if i == 0 || s[i - 1] == b'D' {
                for j in 1..n + 2 {
                    dp[i][j] = (dp[i][j - 1] + dp[i + 1][j - 1]) % Self::MOD
                }
            } else {
                dp[i][n - i] = dp[i + 1][n - i];
                for j in (0..n - i).rev() {
                    dp[i][j] = (dp[i][j + 1] + dp[i + 1][j]) % Self::MOD
                }
            }
        }
        dp[0][n + 1]
    }

    const MOD: i32 = 1_000_000_007;
}
