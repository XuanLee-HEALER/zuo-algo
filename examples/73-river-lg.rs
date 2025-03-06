use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut v = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                v = Vec::with_capacity(n);
                head = true;
            } else {
                v.push(buf.trim().parse().unwrap());
                n -= 1;
                if n == 0 {
                    bw.write_fmt(format_args!("{}\n", compute(&mut v))).unwrap();
                    break;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn compute(times: &mut [i32]) -> i32 {
    let n = times.len();
    times.sort_unstable();
    let mut dp = vec![0; n];
    let mut i = 0;
    while i < n {
        dp[i] = if i == 0 {
            times[i]
        } else if i == 1 {
            times[i]
        } else if i == 2 {
            times[i] + times[0] + times[1]
        } else {
            (times[i] + times[0] + dp[i - 1])
                .min(times[1] + times[1] + times[i] + times[0] + dp[i - 2])
        };
        i += 1;
    }
    dp[n - 1]
}
