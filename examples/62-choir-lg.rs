use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                head = true;
            } else {
                let arr: Vec<i32> = buf
                    .trim()
                    .split(" ")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                if arr.len() == 1 {
                    bw.write_all(b"1\n").unwrap();
                } else {
                    bw.write_fmt(format_args!("{}\n", compute(&arr))).unwrap();
                }
                break;
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

const MOD: i32 = 19_650_827;

fn compute(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut dp = vec![(0, 0); n];
    dp[n - 1] = if arr[n - 2] < arr[n - 1] {
        (1, 1)
    } else {
        (0, 0)
    };
    for i in (0..n.saturating_sub(2)).rev() {
        if arr[i] < arr[i + 1] {
            dp[i + 1] = (1, 1);
        }
        for j in i + 2..n {
            let p1 = (if arr[i] < arr[i + 1] { dp[j].0 } else { 0 }) % MOD;
            let p2 = (if arr[i] < arr[j] { dp[j].1 } else { 0 }) % MOD;
            let p3 = (if arr[j] > arr[j - 1] { dp[j - 1].1 } else { 0 }) % MOD;
            let p4 = (if arr[j] > arr[i] { dp[j - 1].0 } else { 0 }) % MOD;
            dp[j] = ((p1 + p2) % MOD, (p3 + p4) % MOD)
        }
    }
    (dp[n - 1].0 + dp[n - 1].1) % MOD
}
