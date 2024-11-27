use std::i32;

fn main() {}

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // Self::mps_1(&grid, grid.len() - 1, grid[0].len() - 1)
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![-1; m]; n];
        Self::mps_2(&grid, n - 1, m - 1, &mut dp)
    }

    // 暴力递归，返回从(0,0)到(i,j)位置的最短路径
    fn mps_1(grid: &[Vec<i32>], i: usize, j: usize) -> i32 {
        if i == 0 && j == 0 {
            grid[i][j]
        } else {
            let mut up = i32::MAX;
            let mut left: i32 = i32::MAX;
            if i > 0 {
                up = Self::mps_1(grid, i - 1, j)
            }
            if j > 0 {
                left = Self::mps_1(grid, i, j - 1)
            }
            up.min(left) + grid[i][j]
        }
    }

    // 记忆化搜索
    fn mps_2(grid: &[Vec<i32>], i: usize, j: usize, dp: &mut [Vec<i32>]) -> i32 {
        if dp[i][j] != -1 {
            dp[i][j]
        } else if i == 0 && j == 0 {
            dp[i][j] = grid[i][j];
            dp[i][j]
        } else {
            let mut up = i32::MAX;
            let mut left: i32 = i32::MAX;
            if i > 0 {
                up = Self::mps_2(grid, i - 1, j, dp)
            }
            if j > 0 {
                left = Self::mps_2(grid, i, j - 1, dp)
            }
            dp[i][j] = up.min(left) + grid[i][j];
            dp[i][j]
        }
    }

    fn min_path_sum_1(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = grid[0][0];
        for i in 1..n {
            dp[i][0] = dp[i - 1][0] + grid[i][0]
        }
        for j in 1..m {
            dp[0][j] = dp[0][j - 1] + grid[0][j]
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j]
            }
        }
        dp[n - 1][m - 1]
    }

    fn min_path_sum_2(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![0; m];
        dp[0] = grid[0][0];
        for j in 1..m {
            dp[j] = dp[j - 1] + grid[0][j]
        }
        #[allow(clippy::needless_range_loop)]
        for i in 1..n {
            dp[0] += grid[i][0];
            for j in 1..m {
                dp[j] = dp[j].min(dp[j - 1]) + grid[i][j]
            }
        }

        dp[m - 1]
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let n = board.len();
        let m = board[0].len();
        let mut board = board;
        for i in 0..n {
            for j in 0..m {
                if Self::e(&mut board, i as i32, j as i32, word.as_bytes(), 0) {
                    return true;
                }
            }
        }
        false
    }

    fn e(board: &mut [Vec<char>], i: i32, j: i32, word: &[u8], k: usize) -> bool {
        if k >= word.len() {
            true
        } else if i < 0
            || j < 0
            || i >= board.len() as i32
            || j >= board[0].len() as i32
            || board[i as usize][j as usize] as u8 != word[k]
        {
            false
        } else {
            let mut res = false;
            let cur = board[i as usize][j as usize];
            board[i as usize][j as usize] = '0';
            res = res
                || Self::e(board, i + 1, j, word, k + 1)
                || Self::e(board, i - 1, j, word, k + 1)
                || Self::e(board, i, j + 1, word, k + 1)
                || Self::e(board, i, j - 1, word, k + 1);
            board[i as usize][j as usize] = cur;
            res
        }
    }

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // Self::lcs_1(
        //     text1.as_bytes(),
        //     text2.as_bytes(),
        //     (text1.len() - 1) as i32,
        //     (text2.len() - 1) as i32,
        // )
        // Self::lcs_2(text1.as_bytes(), text2.as_bytes(), text1.len(), text2.len())
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![vec![-1; m + 1]; n + 1];
        Self::lcs_3(text1.as_bytes(), text2.as_bytes(), n, m, &mut dp)
    }

    fn lcs_1(t1: &[u8], t2: &[u8], i1: i32, i2: i32) -> i32 {
        if i1 < 0 || i2 < 0 {
            0
        } else {
            let res1 = Self::lcs_1(t1, t2, i1 - 1, i2 - 1);
            let res2 = Self::lcs_1(t1, t2, i1, i2 - 1);
            let res3 = Self::lcs_1(t1, t2, i1 - 1, i2);
            let res4 = if t1[i1 as usize] == t2[i2 as usize] {
                1 + res1
            } else {
                0
            };
            res1.max(res2).max(res3).max(res4)
        }
    }

    fn lcs_2(t1: &[u8], t2: &[u8], l1: usize, l2: usize) -> i32 {
        if l1 == 0 || l2 == 0 {
            0
        } else if t1[l1 - 1] == t2[l2 - 1] {
            1 + Self::lcs_2(t1, t2, l1 - 1, l2 - 1)
        } else {
            Self::lcs_2(t1, t2, l1 - 1, l2).max(Self::lcs_2(t1, t2, l1, l2 - 1))
        }
    }

    fn lcs_3(t1: &[u8], t2: &[u8], l1: usize, l2: usize, dp: &mut [Vec<i32>]) -> i32 {
        if dp[l1][l2] != -1 {
            dp[l1][l2]
        } else if l1 == 0 || l2 == 0 {
            dp[l1][l2] = 0;
            dp[l1][l2]
        } else if t1[l1 - 1] == t2[l2 - 1] {
            dp[l1][l2] = 1 + Self::lcs_3(t1, t2, l1 - 1, l2 - 1, dp);
            dp[l1][l2]
        } else {
            dp[l1][l2] =
                Self::lcs_3(t1, t2, l1 - 1, l2, dp).max(Self::lcs_3(t1, t2, l1, l2 - 1, dp));
            dp[l1][l2]
        }
    }

    // dp
    pub fn longest_common_subsequence_1(text1: String, text2: String) -> i32 {
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        #[allow(clippy::needless_range_loop)]
        for i in 1..=n {
            for j in 1..=m {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1])
                }
            }
        }
        dp[n][m]
    }

    // 优化空间
    pub fn longest_common_subsequence_2(text1: String, text2: String) -> i32 {
        let (text1, text2) = if text1.len() > text2.len() {
            (text1, text2)
        } else {
            (text2, text1)
        };
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![0; m + 1];
        for i in 1..=n {
            let mut lr = 0;
            let mut bk;
            for j in 1..=m {
                bk = dp[j];
                if text1[i - 1] == text2[j - 1] {
                    dp[j] = lr + 1
                } else {
                    dp[j] = dp[j].max(dp[j - 1])
                }
                lr = bk;
            }
        }
        dp[m]
    }

    pub fn longest_palindrome_subseq(s: String) -> i32 {
        // Self::lps_1(s.as_bytes(), 0, s.len() - 1)
        let n = s.len();
        let mut dp = vec![vec![-1; n]; n];
        Self::lps_2(s.as_bytes(), 0, s.len() - 1, &mut dp)
    }

    fn lps_1(s: &[u8], l: usize, r: usize) -> i32 {
        if l == r {
            1
        } else if l + 1 == r {
            if s[l] == s[r] {
                2
            } else {
                1
            }
        } else if s[l] == s[r] {
            2 + Self::lps_1(s, l + 1, r - 1)
        } else {
            Self::lps_1(s, l + 1, r).max(Self::lps_1(s, l, r - 1))
        }
    }

    fn lps_2(s: &[u8], l: usize, r: usize, dp: &mut [Vec<i32>]) -> i32 {
        if dp[l][r] != -1 {
            dp[l][r]
        } else {
            dp[l][r] = if l == r {
                1
            } else if l + 1 == r {
                if s[l] == s[r] {
                    2
                } else {
                    1
                }
            } else if s[l] == s[r] {
                2 + Self::lps_2(s, l + 1, r - 1, dp)
            } else {
                Self::lps_2(s, l + 1, r, dp).max(Self::lps_2(s, l, r - 1, dp))
            };
            dp[l][r]
        }
    }

    pub fn longest_palindrome_subseq_1(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for l in (0..n).rev() {
            let mut r = l;
            if r == l {
                dp[l][r] = 1;
                r += 1;
            }
            if r < n {
                dp[l][r] = if s[l] == s[r] { 2 } else { 1 };
            }
            while r < n {
                dp[l][r] = if s[l] == s[r] {
                    2 + dp[l + 1][r - 1]
                } else {
                    dp[l + 1][r].max(dp[l][r - 1])
                };
                r += 1;
            }
        }

        dp[0][n - 1]
    }

    pub fn longest_palindrome_subseq_2(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![0; n];
        for l in (0..n).rev() {
            let mut r = l;
            if r == l {
                dp[r] = 1;
                r += 1;
            }
            let mut ld = 0;
            if r < n {
                ld = dp[r];
                dp[r] = if s[l] == s[r] { 2 } else { 1 };
                r += 1;
            }

            while r < n {
                let bk = dp[r];
                dp[r] = if s[l] == s[r] {
                    2 + ld
                } else {
                    dp[r].max(dp[r - 1])
                };
                r += 1;
                ld = bk;
            }
        }
        dp[n - 1]
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = vec![vec![0; m]; n];
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                res = res.max(Self::lip(&matrix, i, j, &mut dp))
            }
        }
        res
    }

    fn lip(matrix: &[Vec<i32>], i: usize, j: usize, dp: &mut [Vec<i32>]) -> i32 {
        if dp[i][j] != 0 {
            dp[i][j]
        } else {
            dp[i][j] = {
                let mut res = 0;
                if i > 0 && matrix[i - 1][j] > matrix[i][j] {
                    res = res.max(Self::lip(matrix, i - 1, j, dp))
                }
                if i + 1 < matrix.len() && matrix[i + 1][j] > matrix[i][j] {
                    res = res.max(Self::lip(matrix, i + 1, j, dp))
                }
                if j > 0 && matrix[i][j - 1] > matrix[i][j] {
                    res = res.max(Self::lip(matrix, i, j - 1, dp))
                }
                if j + 1 < matrix[0].len() && matrix[i][j + 1] > matrix[i][j] {
                    res = res.max(Self::lip(matrix, i, j + 1, dp))
                }
                res + 1
            };
            dp[i][j]
        }
    }
}
