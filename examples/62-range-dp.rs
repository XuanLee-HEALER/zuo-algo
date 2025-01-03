use std::i32;

fn main() {
    println!("res {}", Solution::min_cost(7, vec![1, 3, 4, 5]));
    println!("res {}", Solution::min_cost_1(7, vec![1, 3, 4, 5]));
    println!("res {}", Solution::max_coins(vec![3, 1, 5, 8]));
    println!("res {}", Solution::count_eval("1^0|0|1".into(), 0));
    println!("res {}", Solution::remove_boxes(vec![1, 1, 1]));
    println!("res {}", Solution::merge_stones(vec![3, 5, 1, 2, 6], 3));
    println!(
        "res {}",
        Solution::count_palindromic_subsequences("bddaabdbbccdcdcbbdbddccbaaccabbcacbadbdadbccddccdbdbdbdabdbddcccadddaaddbcbcbabdcaccaacabdbdaccbaacc".into())
    );
}

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![-1; n]; n];
        Self::mi(s, 0, n - 1, &mut dp)
    }

    fn mi(s: &[u8], l: usize, r: usize, dp: &mut [Vec<i32>]) -> i32 {
        if l == r {
            0
        } else if l + 1 == r {
            if s[l] == s[r] {
                0
            } else {
                1
            }
        } else if dp[l][r] != -1 {
            dp[l][r]
        } else {
            dp[l][r] = if s[l] == s[r] {
                Self::mi(s, l + 1, r - 1, dp)
            } else {
                (1 + Self::mi(s, l + 1, r, dp)).min(1 + Self::mi(s, l, r - 1, dp))
            };
            dp[l][r]
        }
    }

    pub fn min_insertions_1(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        if n <= 1 {
            0
        } else if n == 2 {
            if s[0] == s[1] {
                0
            } else {
                1
            }
        } else {
            let mut dp = vec![-1; n];
            dp[n - 2] = 0;
            dp[n - 1] = if s[n - 2] == s[n - 1] { 0 } else { 1 };
            for i in (0..n - 2).rev() {
                dp[i] = 0;
                let mut ld = dp[i + 1];
                dp[i + 1] = if s[i] == s[i + 1] { 0 } else { 1 };
                for j in i + 2..n {
                    let t = dp[j];
                    if s[i] == s[j] {
                        dp[j] = ld
                    } else {
                        dp[j] = (1 + dp[j - 1]).min(1 + dp[j])
                    }
                    ld = t;
                }
            }
            dp[n - 1]
        }
    }

    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n <= 2 {
            true
        } else {
            let total = nums.iter().sum::<i32>();
            let mut dp = vec![vec![-1; n]; n];
            for (i, sub) in dp.iter_mut().enumerate() {
                sub[i] = nums[i];
                if i + 1 < n {
                    sub[i + 1] = nums[i].max(nums[i + 1]);
                }
            }
            for i in (0..n - 2).rev() {
                for j in i + 2..n {
                    let p1 = nums[i] + dp[i + 2][j].min(dp[i + 1][j - 1]);
                    let p2 = nums[j] + dp[i][j - 2].min(dp[i + 1][j - 1]);
                    dp[i][j] = p1.max(p2);
                }
            }
            2 * dp[0][n - 1] - total >= 0
        }
    }

    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        if n <= 2 {
            0
        } else {
            let mut dp = vec![vec![-1; n]; n];
            for (i, sub) in dp.iter_mut().enumerate() {
                sub[i] = 0;
                if i + 1 < n {
                    sub[i + 1] = 0;
                }
            }
            for i in (0..n - 2).rev() {
                for j in i + 2..n {
                    let mut res = i32::MAX;
                    for k in i + 1..j {
                        res = res.min(values[k] * values[i] * values[j] + dp[i][k] + dp[k][j])
                    }
                    dp[i][j] = res
                }
            }
            dp[0][n - 1]
        }
    }

    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts = cuts;
        let m = cuts.len();
        cuts.sort();
        cuts.insert(0, 0);
        cuts.push(n);
        let mut dp = vec![vec![-1; m]; m];
        Self::mc(&cuts, 1, m, &mut dp)
    }

    fn mc(arr: &[i32], l: usize, r: usize, dp: &mut [Vec<i32>]) -> i32 {
        if l > r {
            0
        } else if l == r {
            arr[r + 1] - arr[l - 1]
        } else if dp[l - 1][r - 1] != -1 {
            dp[l - 1][r - 1]
        } else {
            dp[l - 1][r - 1] = i32::MAX;
            for k in l..=r {
                dp[l - 1][r - 1] = dp[l - 1][r - 1].min(
                    arr[r + 1] - arr[l - 1]
                        + Self::mc(arr, l, k - 1, dp)
                        + Self::mc(arr, k + 1, r, dp),
                )
            }
            dp[l - 1][r - 1]
        }
    }

    pub fn min_cost_1(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts = cuts;
        let m = cuts.len();
        cuts.sort();
        cuts.insert(0, 0);
        cuts.push(n);
        let mut dp = vec![vec![i32::MAX; m + 2]; m + 2];
        for (i, sub) in dp.iter_mut().skip(1).take(m).enumerate() {
            sub[i + 1] = cuts[i + 2] - cuts[i];
        }
        for i in (1..m).rev() {
            for j in i + 1..=m {
                for k in i..=j {
                    dp[i][j] = dp[i][j].min(
                        cuts[j + 1] - cuts[i - 1]
                            + if i > k - 1 { 0 } else { dp[i][k - 1] }
                            + if k + 1 > j { 0 } else { dp[k + 1][j] },
                    )
                }
            }
        }
        dp[1][m]
    }

    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let m = nums.len();
        nums.insert(0, 1);
        nums.push(1);
        let n = nums.len();
        let mut dp = vec![vec![-1; n]; n];
        for (i, sub) in dp.iter_mut().skip(1).take(m).enumerate() {
            sub[i + 1] = nums[i] * nums[i + 1] * nums[i + 2]
        }
        for i in (1..n - 1).rev() {
            for j in i + 1..=m {
                let mut res = (nums[i - 1] * nums[i] * nums[j + 1] + dp[i + 1][j])
                    .max(nums[i - 1] * nums[j] * nums[j + 1] + dp[i][j - 1]);
                for k in i + 1..j {
                    res = res.max(dp[i][k - 1] + nums[i - 1] * nums[k] * nums[j + 1] + dp[k + 1][j])
                }
                dp[i][j] = res
            }
        }
        dp[1][m]
    }

    pub fn count_eval(s: String, result: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![(-1, -1); n]; n];
        let res = Self::ce(s, 0, n - 1, &mut dp);
        if result == 1 {
            res.1
        } else {
            res.0
        }
    }

    // l..=r范围内，l和r都是0/1的情况下，计算结果是0或者1的所有可能性
    fn ce(s: &[u8], l: usize, r: usize, dp: &mut [Vec<(i32, i32)>]) -> (i32, i32) {
        if l == r {
            if s[l] == b'0' {
                (1, 0)
            } else {
                (0, 1)
            }
        } else if dp[l][r] != (-1, -1) {
            dp[l][r]
        } else {
            let mut res = (0, 0);
            for k in (l + 1..r).step_by(2) {
                let left = Self::ce(s, l, k - 1, dp);
                let right = Self::ce(s, k + 1, r, dp);
                let t_res = match s[k] {
                    b'&' => (
                        left.0 * right.0 + left.1 * right.0 + left.0 * right.1,
                        left.1 * right.1,
                    ),
                    b'|' => (
                        left.0 * right.0,
                        left.0 * right.1 + left.1 * right.0 + left.1 * right.1,
                    ),
                    b'^' => (
                        left.0 * right.0 + left.1 * right.1,
                        left.0 * right.1 + left.1 * right.0,
                    ),
                    _ => panic!("error"),
                };
                res = (res.0 + t_res.0, res.1 + t_res.1)
            }
            dp[l][r] = res;
            dp[l][r]
        }
    }

    pub fn count_eval_1(s: String, result: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![(-1, -1); n]; n];
        for (i, sub) in dp.iter_mut().enumerate() {
            if s[i] == b'0' {
                sub[i] = (1, 0)
            } else if s[i] == b'1' {
                sub[i] = (0, 1)
            }
        }
        for i in (0..n - 1).rev() {
            if s[i] == b'0' || s[i] == b'1' {
                for j in i + 1..n {
                    if s[j] == b'0' || s[j] == b'1' {
                        let mut res = (0, 0);
                        for k in (i + 1..j).step_by(2) {
                            let left = dp[i][k - 1];
                            let right = dp[k + 1][j];
                            let t_res = match s[k] {
                                b'&' => (
                                    left.0 * right.0 + left.1 * right.0 + left.0 * right.1,
                                    left.1 * right.1,
                                ),
                                b'|' => (
                                    left.0 * right.0,
                                    left.0 * right.1 + left.1 * right.0 + left.1 * right.1,
                                ),
                                b'^' => (
                                    left.0 * right.0 + left.1 * right.1,
                                    left.0 * right.1 + left.1 * right.0,
                                ),
                                _ => panic!("error"),
                            };
                            res = (res.0 + t_res.0, res.1 + t_res.1)
                        }
                        dp[i][j] = res
                    }
                }
            }
        }
        if result == 0 {
            dp[0][n - 1].0
        } else {
            dp[0][n - 1].1
        }
    }

    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut dp = vec![vec![vec![0; n]; n]; n];
        Self::rb(&boxes, 0, n - 1, 0, &mut dp)
    }

    fn rb(arr: &[i32], l: usize, r: usize, k: usize, dp: &mut [Vec<Vec<i32>>]) -> i32 {
        if l > r {
            0
        } else if dp[l][r][k] > 0 {
            dp[l][r][k]
        } else {
            let mut s = l;
            while s < r && arr[s + 1] == arr[s] {
                s += 1
            }
            let cn = s - l + 1 + k;
            let mut p1 = (cn as i32 * cn as i32) + Self::rb(arr, s + 1, r, 0, dp);
            let mut p = s + 1;
            while p <= r {
                if arr[p] == arr[l] && arr[p] != arr[p - 1] {
                    p1 = p1.max(Self::rb(arr, s + 1, p - 1, 0, dp) + Self::rb(arr, p, r, cn, dp))
                }
                p += 1;
            }
            dp[l][r][k] = p1;
            dp[l][r][k]
        }
    }

    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        if (n as i32 - 1) % (k - 1) != 0 {
            -1
        } else {
            let mut presum = vec![0; n + 1];
            let mut sum = 0;
            for (i, &stone) in stones.iter().enumerate() {
                presum[i + 1] = sum + stone;
                sum += stone;
            }

            let mut dp = vec![vec![0; n]; n];
            for i in (0..n - 1).rev() {
                for j in i + 1..n {
                    let mut tk = i;
                    let mut res = i32::MAX;
                    while tk < j {
                        res = res.min(dp[i][tk] + dp[tk + 1][j]);
                        tk += k as usize - 1
                    }
                    if (j as i32 - i as i32) % (k - 1) == 0 {
                        res = res + presum[j + 1] - presum[i]
                    }
                    dp[i][j] = res
                }
            }
            dp[0][n - 1]
        }
    }

    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0_i64; n]; n];
        for (i, sub) in dp.iter_mut().enumerate() {
            sub[i] = 1
        }
        // 左边距离 i 位置字符最近的相同字符位置，初始为-1
        let mut left = vec![-1; n];
        // 右边距离 i 位置字符最近的相同字符位置，初始为n
        let mut right = vec![n as i32; n];
        let mut ai = -1;
        let mut bi = -1;
        let mut ci = -1;
        let mut di = -1;

        for (i, &b) in s.iter().enumerate() {
            match b {
                b'a' => {
                    left[i] = ai;
                    ai = i as i32
                }
                b'b' => {
                    left[i] = bi;
                    bi = i as i32
                }
                b'c' => {
                    left[i] = ci;
                    ci = i as i32
                }
                b'd' => {
                    left[i] = di;
                    di = i as i32
                }
                _ => (),
            }
        }
        ai = n as i32;
        bi = n as i32;
        ci = n as i32;
        di = n as i32;
        for (i, &b) in s.iter().rev().enumerate() {
            match b {
                b'a' => {
                    right[n - i - 1] = ai;
                    ai = (n - i - 1) as i32
                }
                b'b' => {
                    right[n - i - 1] = bi;
                    bi = (n - i - 1) as i32
                }
                b'c' => {
                    right[n - i - 1] = ci;
                    ci = (n - i - 1) as i32
                }
                b'd' => {
                    right[n - i - 1] = di;
                    di = (n - i - 1) as i32
                }
                _ => (),
            }
        }
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if s[i] != s[j] {
                    dp[i][j] = (dp[i][j - 1] + dp[i + 1][j] - dp[i + 1][j - 1] + MOD) % MOD
                } else {
                    match left[j].cmp(&right[i]) {
                        std::cmp::Ordering::Less => dp[i][j] = (2 * dp[i + 1][j - 1] + 2) % MOD,
                        std::cmp::Ordering::Equal => dp[i][j] = (2 * dp[i + 1][j - 1] + 1) % MOD,
                        std::cmp::Ordering::Greater => {
                            let (ti, tj) = (right[i] as usize, left[j] as usize);
                            dp[i][j] = (2 * dp[i + 1][j - 1] - dp[ti + 1][tj - 1] + MOD) % MOD
                        }
                    }
                }
            }
        }

        (dp[0][n - 1] % MOD) as i32
    }
}
