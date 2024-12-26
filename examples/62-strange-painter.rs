use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    if let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            bw.write_fmt(format_args!("{}\n", compute(buf.trim())))
                .unwrap();
        }
    }
    bw.flush().unwrap();
}

fn compute(s: &str) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![vec![-1; n]; n];
    for (i, sub) in dp.iter_mut().enumerate() {
        sub[i] = 1;
        if i + 1 < n {
            sub[i + 1] = if s[i] == s[i + 1] { 1 } else { 2 }
        }
    }
    for i in (0..n.saturating_sub(2)).rev() {
        for j in i + 2..n {
            dp[i][j] = if s[i] == s[j] {
                dp[i][j - 1]
            } else {
                let mut p = i32::MAX;
                for k in i..j {
                    p = p.min(dp[i][k] + dp[k + 1][j])
                }
                p
            }
        }
    }
    dp[0][n - 1]
}
