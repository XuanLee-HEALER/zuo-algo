use std::usize;

fn main() {
    println!(
        "res {}",
        Solution::max_profit_iv_2(2, vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0])
    );
    println!("res {}", Solution::max_profit_vi(vec![2, 1, 4]));
    println!("res {}", Solution::num_perms_di_sequence_2("DID".into()));
    println!(
        "res {}",
        Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4])
    );
    assert_eq!(
        Solution::k_inverse_pairs(500, 50),
        Solution::k_inverse_pairs_1(500, 50)
    );
    println!(
        "res {}",
        Solution::find_rotate_steps("godding".into(), "gd".into())
    )
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

    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = profit.len();
        let mut sched = Vec::with_capacity(n);
        for i in 0..n {
            sched.push((start_time[i], end_time[i], profit[i]));
        }
        sched.sort_unstable_by_key(|s| s.1);
        let mut dp = vec![0; n];
        dp[0] = sched[0].2;
        for i in 1..n {
            let mut res = dp[i - 1];
            if sched[0].1 <= sched[i].0 {
                res = res.max(dp[Self::find(&sched[0..i], sched[i].0)] + sched[i].2)
            } else {
                res = res.max(sched[i].2)
            }
            dp[i] = res
        }
        dp[n - 1]
    }

    // 小于等于v的最右位置
    fn find(arr: &[(i32, i32, i32)], v: i32) -> usize {
        let n = arr.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut res = 0;
        while l <= r {
            let mid = l + ((r - l) >> 1);
            if arr[mid].1 > v {
                if mid > 1 {
                    r = mid - 1
                } else {
                    break;
                }
            } else {
                res = mid;
                l = mid + 1;
            }
        }
        res
    }

    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; n + 1];
        dp[1][0] = 1;
        for i in 2..=n {
            dp[i][0] = 1;
            for j in 1..=k {
                let mut res = 0;
                if i > j {
                    for k in 0..=j {
                        res = (res + dp[i - 1][k]) % Self::MOD;
                    }
                } else {
                    for k in (j - i + 1)..=j {
                        res = (res + dp[i - 1][k]) % Self::MOD;
                    }
                }
                dp[i][j] = res
            }
        }
        dp[n][k]
    }

    pub fn k_inverse_pairs_1(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut old = vec![0; k + 1];
        let mut dp = vec![0; k + 1];
        let mut old = &mut old;
        let mut dp = &mut dp;
        old[0] = 1;
        dp[0] = 1;
        for i in 1..=n {
            let mut window = old[0];
            for j in 1..=k {
                dp[j] = if i > j {
                    window = (old[j] + window) % Self::MOD;
                    window
                } else {
                    window = ((old[j] + window) % Self::MOD - old[j - i] + Self::MOD) % Self::MOD;
                    window
                }
            }
            let tmp = old;
            old = dp;
            dp = tmp;
        }
        old[k]
    }

    const MOD: i32 = 1_000_000_007;

    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.as_bytes();
        let key = key.as_bytes();
        let mut arr1 = Vec::with_capacity(ring.len());
        let mut arr2 = Vec::with_capacity(key.len());
        let mut statis = vec![vec![]; 26];
        for (i, &b) in ring.iter().enumerate() {
            let cur = (b - b'a') as usize;
            arr1.push(cur);
            statis[cur].push(i);
        }
        for &b in key {
            arr2.push((b - b'a') as usize);
        }
        let n = arr1.len();
        let m = arr2.len();
        let mut dp = vec![vec![-1; m]; n];
        Self::frs(&arr1, &arr2, arr1.len(), arr2.len(), &statis, 0, 0, &mut dp)
    }

    fn frs(
        arr1: &[usize],
        arr2: &[usize],
        n: usize,
        m: usize,
        statis: &[Vec<usize>],
        i: usize,
        j: usize,
        dp: &mut [Vec<i32>],
    ) -> i32 {
        if j == m {
            0
        } else if dp[i][j] != -1 {
            dp[i][j]
        } else if arr1[i] == arr2[j] {
            1 + Self::frs(arr1, arr2, n, m, statis, i, j + 1, dp)
        } else if statis[arr2[j]].len() > 0 {
            let mut res = i32::MAX;
            // 从i开始顺时针第一个j字符，就是statis[arr2[j]]大于i的最左
            res = if let Some(c) = Self::clock(&statis[arr2[j]], i) {
                res.min((c - i) as i32 + Self::frs(arr1, arr2, n, m, statis, c, j, dp))
            } else {
                let &c = statis[arr2[j]].first().unwrap();
                res.min((n - i + c) as i32 + Self::frs(arr1, arr2, n, m, statis, c, j, dp))
            };
            // 从i开始逆时针第一个j字符，就是statis[arr2[j]]小于i的最右
            res = if let Some(c) = Self::counter_clock(&statis[arr2[j]], i) {
                res.min((i - c) as i32 + Self::frs(arr1, arr2, n, m, statis, c, j, dp))
            } else {
                let &c = statis[arr2[j]].last().unwrap();
                res.min((n - c + i) as i32 + Self::frs(arr1, arr2, n, m, statis, c, j, dp))
            };
            dp[i][j] = res;
            dp[i][j]
        } else {
            0
        }
    }

    // 大于v的最左
    fn clock(arr: &[usize], v: usize) -> Option<usize> {
        let n = arr.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut res = None;
        while l <= r {
            let mid = l + ((r - l) >> 1);
            if arr[mid] > v {
                res = Some(arr[mid]);
                if mid > 0 {
                    r = mid - 1;
                } else {
                    break;
                }
            } else {
                l = mid + 1;
            }
        }
        res
    }

    // 小于v的最右
    fn counter_clock(arr: &[usize], v: usize) -> Option<usize> {
        let n = arr.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut res = None;
        while l <= r {
            let mid = l + ((r - l) >> 1);
            if arr[mid] < v {
                res = Some(arr[mid]);
                l = mid + 1;
            } else if mid > 0 {
                r = mid - 1;
            } else {
                break;
            }
        }
        res
    }
}
