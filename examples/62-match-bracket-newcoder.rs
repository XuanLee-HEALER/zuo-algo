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
            if (s[i] == b'(' && s[i + 1] == b')') || (s[i] == b'[' && s[i + 1] == b']') {
                sub[i + 1] = 0
            } else {
                sub[i + 1] = 2
            }
        }
    }
    for i in (0..n - 2).rev() {
        for j in i + 2..n {
            let mut p1 = i32::MAX;
            if (s[i] == b'(' && s[j] == b')') || (s[i] == b'[' && s[j] == b']') {
                p1 = p1.min(dp[i + 1][j - 1])
            }

            for k in i..j {
                p1 = p1.min(dp[i][k] + dp[k + 1][j])
            }
            dp[i][j] = p1
        }
    }
    dp[0][n - 1]
}
