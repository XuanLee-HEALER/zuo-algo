use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MOD: i32 = 1_000_000_007;
const MAX_N: i32 = 100_000;

fn main() {
    let mut ct = vec![0; (MAX_N + 1) as usize];
    let mut max = 0;
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    for i in buf.trim().split(" ").map(|v| v.parse::<i32>().unwrap()) {
        ct[i as usize] += 1;
        max = max.max(i);
    }
    let mut pow2 = vec![1; (MAX_N + 1) as usize];
    let mut res = 1;
    for i in 1..=MAX_N {
        res = (res as i64 * 2 as i64 % MOD as i64) as i32;
        pow2[i as usize] = res;
    }
    let mut dp = vec![0; (max + 1) as usize];
    for i in (1..=max).rev() {
        let mut cts = ct[i as usize];
        let mut j = 2;
        while i * j <= max {
            cts += ct[(i * j) as usize];
            j += 1;
        }
        dp[i as usize] = (pow2[cts as usize] as i64 - 1_i64 + MOD as i64) % MOD as i64;
        j = 2;
        while i * j <= max {
            dp[i as usize] =
                (dp[i as usize] as i64 - dp[(i * j) as usize] as i64 + MOD as i64) % MOD as i64;
            j += 1;
        }
    }
    bw.write_fmt(format_args!("{}\n", dp[1],)).unwrap();
    bw.flush().unwrap();
}
