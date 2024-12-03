use std::mem;

fn main() {
    println!("res: {}", Solution::knight_probability_1(3, 2, 0, 0));
    println!(
        "res1: {}",
        Solution::number_of_paths_1(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3)
    );
    println!(
        "scramble: {}",
        Solution::is_scramble("great".into(), "rgeat".into())
    );
    println!(
        "scramble_1: {}",
        Solution::is_scramble_1("abcdbdacbdac".into(), "bdacabcdbdac".into())
    );
}

struct Solution;

impl Solution {
    fn compute_zero_one(x: &str) -> (usize, usize) {
        let (mut zero, mut one) = (0, 0);
        x.as_bytes()
            .iter()
            .for_each(|&i| if i == b'0' { zero += 1 } else { one += 1 });
        (zero, one)
    }

    // n个1，m个0
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let i = strs.len();
        let mut dp = vec![vec![vec![-1; n + 1]; m + 1]; i];
        Self::fmf(&strs, 0, m, n, &mut dp)
    }

    fn fmf(strs: &[String], i: usize, j: usize, k: usize, dp: &mut [Vec<Vec<i32>>]) -> i32 {
        if i == strs.len() {
            0
        } else if dp[i][j][k] != -1 {
            dp[i][j][k]
        } else {
            let r1 = Self::fmf(strs, i + 1, j, k, dp);
            let (zero, one) = Self::compute_zero_one(&strs[i]);
            let r2 = if zero <= j && one <= k {
                1 + Self::fmf(strs, i + 1, j - zero, k - one, dp)
            } else {
                0
            };
            dp[i][j][k] = r1.max(r2);
            dp[i][j][k]
        }
    }

    pub fn find_max_form_1(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let i = strs.len();
        let mut dp = vec![vec![vec![0; n + 1]; m + 1]; i + 1];
        for ti in (0..i).rev() {
            let (zero, one) = Self::compute_zero_one(&strs[ti]);
            for j in 0..=m {
                for k in 0..=n {
                    dp[ti][j][k] = if zero <= j && one <= k {
                        (1 + dp[ti + 1][j - zero][k - one]).max(dp[ti + 1][j][k])
                    } else {
                        dp[ti + 1][j][k]
                    }
                }
            }
        }
        dp[0][m][n]
    }

    pub fn find_max_form_2(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for s in strs.iter() {
            let (zero, one) = Self::compute_zero_one(s);
            for j in (zero..=m).rev() {
                for k in (one..=n).rev() {
                    dp[j][k] = dp[j][k].max(1 + dp[j - zero][k - one]);
                }
            }
        }
        dp[m][n]
    }

    const MOD: i32 = 1_000_000_007;

    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let i = group.len();
        let j = n as usize;
        let k = min_profit as usize;
        let mut dp = vec![vec![vec![-1; k + 1]; j + 1]; i];
        Self::ps(&group, &profit, 0, j, k, &mut dp)
    }

    // 从第i个任务开始，要满足不超过j个工人且产生至少为k利润的条件所构成的盈利计划的数量
    fn ps(g: &[i32], p: &[i32], i: usize, j: usize, k: usize, dp: &mut [Vec<Vec<i32>>]) -> i32 {
        if j == 0 || i == g.len() {
            if k == 0 {
                1
            } else {
                0
            }
        } else if dp[i][j][k] != -1 {
            dp[i][j][k]
        } else {
            let p1 = Self::ps(g, p, i + 1, j, k, dp);
            let p2 = if j >= g[i] as usize {
                Self::ps(
                    g,
                    p,
                    i + 1,
                    j - g[i] as usize,
                    if k >= p[i] as usize {
                        k - p[i] as usize
                    } else {
                        0
                    },
                    dp,
                )
            } else {
                0
            };
            dp[i][j][k] = ((p1 as i64 + p2 as i64) % Self::MOD as i64) as i32;
            dp[i][j][k]
        }
    }

    pub fn profitable_schemes_1(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let i = group.len();
        let j = n as usize;
        let k = min_profit as usize;
        let mut dp = vec![vec![0; k + 1]; j + 1];
        // 第i层的值
        for sub in dp.iter_mut() {
            sub[0] = 1
        }

        for ti in (0..i).rev() {
            for tk in (0..=k).rev() {
                for tj in (0..=j).rev() {
                    dp[tj][tk] = (((dp[tj][tk] as i64)
                        + ((if tj >= group[ti] as usize {
                            dp[tj - group[ti] as usize][if tk >= profit[ti] as usize {
                                tk - profit[ti] as usize
                            } else {
                                0
                            }]
                        } else {
                            0
                        }) as i64))
                        % Self::MOD as i64) as i32
                }
            }
        }

        dp[j][k]
    }

    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut dp = vec![vec![vec![-1_f64; k as usize + 1]; n as usize]; n as usize];
        Self::kp(n, row, column, k, &mut dp)
    }

    fn kp(n: i32, i: i32, j: i32, k: i32, dp: &mut [Vec<Vec<f64>>]) -> f64 {
        if i < 0 || i >= n || j < 0 || j >= n {
            0_f64
        } else if k == 0 {
            1_f64
        } else if dp[i as usize][j as usize][k as usize] != -1_f64 {
            dp[i as usize][j as usize][k as usize]
        } else {
            dp[i as usize][j as usize][k as usize] = (Self::kp(n, i - 2, j + 1, k - 1, dp)
                + Self::kp(n, i - 1, j + 2, k - 1, dp)
                + Self::kp(n, i + 1, j + 2, k - 1, dp)
                + Self::kp(n, i + 2, j + 1, k - 1, dp)
                + Self::kp(n, i + 2, j - 1, k - 1, dp)
                + Self::kp(n, i + 1, j - 2, k - 1, dp)
                + Self::kp(n, i - 1, j - 2, k - 1, dp)
                + Self::kp(n, i - 2, j - 1, k - 1, dp))
                / 8_f64;
            dp[i as usize][j as usize][k as usize]
        }
    }

    pub fn knight_probability_1(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let valid_idx = |idx: (i32, i32)| idx.0 >= 0 && idx.0 < n && idx.1 >= 0 && idx.1 < n;
        let mut dp = vec![vec![1_f64; n as usize]; n as usize];
        let mut aid = dp.clone();
        for _ in 1..=k {
            mem::swap(&mut dp, &mut aid);
            for j in 0..n {
                for k in 0..n {
                    let i1 = (j - 2, k + 1);
                    let i2 = (j - 1, k + 2);
                    let i3 = (j + 1, k + 2);
                    let i4 = (j + 2, k + 1);
                    let i5 = (j + 2, k - 1);
                    let i6 = (j + 1, k - 2);
                    let i7 = (j - 1, k - 2);
                    let i8 = (j - 2, k - 1);
                    dp[j as usize][k as usize] = ((if valid_idx(i1) {
                        aid[i1.0 as usize][i1.1 as usize]
                    } else {
                        0_f64
                    }) + (if valid_idx(i2) {
                        aid[i2.0 as usize][i2.1 as usize]
                    } else {
                        0_f64
                    }) + (if valid_idx(i3) {
                        aid[i3.0 as usize][i3.1 as usize]
                    } else {
                        0_f64
                    }) + (if valid_idx(i4) {
                        aid[i4.0 as usize][i4.1 as usize]
                    } else {
                        0_f64
                    }) + (if valid_idx(i5) {
                        aid[i5.0 as usize][i5.1 as usize]
                    } else {
                        0_f64
                    }) + (if valid_idx(i6) {
                        aid[i6.0 as usize][i6.1 as usize]
                    } else {
                        0_f64
                    }) + (if valid_idx(i7) {
                        aid[i7.0 as usize][i7.1 as usize]
                    } else {
                        0_f64
                    }) + (if valid_idx(i8) {
                        aid[i8.0 as usize][i8.1 as usize]
                    } else {
                        0_f64
                    })) / 8_f64
                }
            }
        }
        dp[row as usize][column as usize]
    }

    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![vec![-1; k as usize]; m]; n];
        Self::nop(&grid, n, m, k, 0, 0, 0, &mut dp)
    }

    #[allow(clippy::too_many_arguments)]
    fn nop(
        grid: &[Vec<i32>],
        n: usize,
        m: usize,
        target: i32,
        i: usize,
        j: usize,
        k: i32,
        dp: &mut [Vec<Vec<i32>>],
    ) -> i32 {
        if i == n - 1 && j == m - 1 {
            if grid[i][j] % target == k {
                1
            } else {
                0
            }
        } else if dp[i][j][k as usize] != -1 {
            dp[i][j][k as usize]
        } else {
            let cur = grid[i][j] % target;
            let need = (target + k - cur) % target;
            let p1 = if i < n - 1 {
                Self::nop(grid, n, m, target, i + 1, j, need, dp)
            } else {
                0
            };
            let p2 = if j < m - 1 {
                Self::nop(grid, n, m, target, i, j + 1, need, dp)
            } else {
                0
            };
            dp[i][j][k as usize] = ((p1 as i64 + p2 as i64) % Self::MOD as i64) as i32;
            dp[i][j][k as usize]
        }
    }

    pub fn number_of_paths_1(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![vec![0; k as usize]; m]; n];
        dp[n - 1][m - 1][(grid[n - 1][m - 1] % k) as usize] = 1;
        for i in (0..n - 1).rev() {
            for tk in 0..k {
                let need = (tk + k - grid[i][m - 1] % k) % k;
                dp[i][m - 1][tk as usize] = dp[i + 1][m - 1][need as usize];
            }
        }
        for j in (0..m - 1).rev() {
            for tk in 0..k {
                let need = (tk + k - grid[n - 1][j] % k) % k;
                dp[n - 1][j][tk as usize] = dp[n - 1][j + 1][need as usize];
            }
        }

        for i in (0..n - 1).rev() {
            for j in (0..m - 1).rev() {
                for tk in 0..k {
                    let need = (tk + k - grid[i][j] % k) % k;
                    let d = dp[i + 1][j][need as usize];
                    let r = dp[i][j + 1][need as usize];
                    dp[i][j][tk as usize] = ((d as i64 + r as i64) % Self::MOD as i64) as i32
                }
            }
        }

        dp[0][0][0]
    }

    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let mut dp = vec![vec![vec![0; n + 1]; n]; n];
        // Self::scramble(s1, s2, 0, s1.len() - 1, 0, s2.len() - 1)
        Self::scramble_1(s1, s2, 0, 0, n, &mut dp)
    }

    // 递归尝试方法，s1[l1..=r1]是否能通过扰乱得到s2[l2..=r2]
    fn scramble(s1: &[u8], s2: &[u8], l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
        if l1 == r1 {
            return s1[l1] == s2[l2];
        }
        for i in 0..(r1 - l1) {
            if (Self::scramble(s1, s2, l1, l1 + i, l2, l2 + i)
                && Self::scramble(s1, s2, l1 + i + 1, r1, l2 + i + 1, r2))
                || (Self::scramble(s1, s2, l1, l1 + i, r2 - i, r2)
                    && Self::scramble(s1, s2, l1 + i + 1, r1, l2, r2 - i - 1))
            {
                return true;
            }
        }

        false
    }

    fn scramble_1(
        s1: &[u8],
        s2: &[u8],
        l1: usize,
        l2: usize,
        l: usize,
        dp: &mut [Vec<Vec<i32>>],
    ) -> bool {
        if l == 1 {
            s1[l1] == s2[l2]
        } else if dp[l1][l2][l] != 0 {
            dp[l1][l2][l] == 1
        } else {
            let mut ans = false;
            for i in 1..l {
                if (Self::scramble_1(s1, s2, l1, l2, i, dp)
                    && Self::scramble_1(s1, s2, l1 + i, l2 + i, l - i, dp))
                    || (Self::scramble_1(s1, s2, l1, l2 + l - i, i, dp)
                        && Self::scramble_1(s1, s2, l1 + i, l2, l - i, dp))
                {
                    ans = true;
                    break;
                }
            }
            dp[l1][l2][l] = if ans { 1 } else { -1 };
            dp[l1][l2][l] == 1
        }
    }

    pub fn is_scramble_1(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let mut dp = vec![vec![vec![false; n]; n]; n + 1];
        for (i, sub) in dp[1].iter_mut().enumerate() {
            for (j, e) in sub.iter_mut().enumerate() {
                *e = s1[i] == s2[j]
            }
        }
        for l in 2..=n {
            for i in 0..=(n - l) {
                for j in 0..=(n - l) {
                    for xl in 1..l {
                        dp[l][i][j] = (dp[xl][i][j] && dp[l - xl][i + xl][j + xl])
                            || (dp[xl][i][j + l - xl] && dp[l - xl][i + xl][j]);
                        // 只要满足就不要再继续更新值
                        if dp[l][i][j] {
                            break;
                        }
                    }
                }
            }
        }
        dp[n][0][0]
    }
}
