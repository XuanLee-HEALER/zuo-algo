use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    if let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            let n = segs.next().unwrap().parse::<usize>().unwrap();
            let m = segs.next().unwrap().parse::<usize>().unwrap();

            bw.write_fmt(format_args!("{}\n", compute2(n, m))).unwrap();
        }
    }
    bw.flush().unwrap()
}

const MOD: i32 = 1_000_000_007;

fn compute(n: usize, m: usize) -> i32 {
    let mut dp = vec![vec![-1; m + 1]; n + 1];
    sub(n, m, &mut dp)
}

fn sub(n: usize, m: usize, dp: &mut [Vec<i32>]) -> i32 {
    if dp[n][m] != -1 {
        dp[n][m]
    } else {
        dp[n][m] = if n == 0 {
            1
        } else if m == 0 {
            0
        } else {
            let mut res = 0;
            for k in 0..=(n - 1) {
                res = (res
                    + ((sub(k, m - 1, dp) as i64 * sub(n - k - 1, m - 1, dp) as i64) % MOD as i64)
                        as i32)
                    % MOD;
            }
            res
        };
        dp[n][m]
    }
}

fn compute1(n: usize, m: usize) -> i32 {
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for e in dp[0].iter_mut() {
        *e = 1
    }
    for i in 1..=n {
        for j in 1..=m {
            let mut res = 0;
            for k in 0..i {
                res = (res
                    + ((dp[k][j - 1] as i64 * dp[i - k - 1][j - 1] as i64) % MOD as i64) as i32)
                    % MOD
            }
            dp[i][j] = res
        }
    }

    dp[n][m]
}

fn compute2(n: usize, m: usize) -> i32 {
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    'out: for j in 1..=m {
        for i in (1..=n).rev() {
            let mut res = 0;
            for k in 0..i {
                res = (res + ((dp[k] as i64 * dp[i - k - 1] as i64) % MOD as i64) as i32) % MOD
            }
            dp[i] = res;
            if j == m && i == n {
                break 'out;
            }
        }
    }
    dp[n]
}
