use std::io::{self, BufRead, BufReader, BufWriter, Write};

const LIMIT: i32 = 100_000;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split(" ");
    let v1: i32 = segs.next().unwrap().parse().unwrap();
    let v2: i32 = segs.next().unwrap().parse().unwrap();
    let v3: i32 = segs.next().unwrap().parse().unwrap();
    let v4: i32 = segs.next().unwrap().parse().unwrap();
    let mut dp = vec![0; LIMIT as usize + 1];
    let coins = vec![v1, v2, v3, v4];
    x_dp(&coins, &mut dp);
    let n: i32 = segs.next().unwrap().parse().unwrap();
    for _ in 0..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split(" ");
        let k1: i32 = segs.next().unwrap().parse().unwrap();
        let k2: i32 = segs.next().unwrap().parse().unwrap();
        let k3: i32 = segs.next().unwrap().parse().unwrap();
        let k4: i32 = segs.next().unwrap().parse().unwrap();
        let s: i32 = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!(
            "{}\n",
            compute(&coins, &vec![k1, k2, k3, k4], s, &dp)
        ))
        .unwrap();
    }
    bw.flush().unwrap();
}

fn compute(coins: &[i32], ks: &[i32], s: i32, dp: &[i64]) -> i64 {
    let all = dp[s as usize];
    let mut t = 0;
    for mut xs in 1..16 {
        let mut sign = -1;
        let mut c = 0;
        for i in 0..4 {
            if xs & 1 != 0 {
                c += coins[i as usize] * (ks[i as usize] + 1);
                sign = -sign;
            }
            xs >>= 1;
        }
        let ct = if s >= c { dp[(s - c) as usize] } else { 0 };
        t += sign * ct;
    }
    all - t
}

fn x_dp(coins: &[i32], dp: &mut [i64]) {
    dp[0] = 1;
    for i in 0..4 {
        dp[0] = 1;
        for j in 1..=LIMIT {
            if j >= coins[i] {
                dp[j as usize] = dp[j as usize] + dp[(j - coins[i]) as usize]
            }
        }
    }
}
