use std::io::{self, BufReader, BufWriter, Read, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    if let Ok(sz) = br.read_to_string(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split("\n");
            let str1 = segs.next().unwrap().as_bytes();
            let str2 = segs.next().unwrap().as_bytes();
            let n = str1.len();
            let m = str2.len();
            let mut dp = vec![vec![0; m + 1]; n + 1];
            let t_res = x(str1, str2, n, m, &mut dp);
            if t_res == 0 {
                bw.write_fmt(format_args!("{}\n", -1)).unwrap()
            } else {
                let mut res = vec![0; t_res as usize];
                let mut i = n;
                let mut j = m;
                let mut k = t_res as usize - 1;
                loop {
                    if str1[i - 1] == str2[j - 1] {
                        res[k] = str1[i - 1];
                        i -= 1;
                        j -= 1;
                        if k == 0 {
                            break;
                        }
                        k -= 1;
                    } else {
                        if dp[i][j - 1] >= dp[i - 1][j] {
                            j -= 1;
                        } else {
                            i -= 1;
                        }
                    }
                }
                bw.write_fmt(format_args!("{}\n", String::from_utf8(res).unwrap()))
                    .unwrap()
            }
        }
    }
    bw.flush().unwrap();
}

fn x(str1: &[u8], str2: &[u8], n: usize, m: usize, dp: &mut [Vec<i32>]) -> i32 {
    for i in 1..=n {
        for j in 1..=m {
            if str1[i - 1] == str2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1]
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1])
            }
        }
    }
    dp[n][m]
}
