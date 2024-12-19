use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut m = 0;
    let mut costs = vec![];
    let mut vals = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                m = segs.next().unwrap().parse().unwrap();
                n = segs.next().unwrap().parse().unwrap();
                costs = Vec::with_capacity(n);
                vals = Vec::with_capacity(n);
                head = true;
            } else {
                let cost = segs.next().unwrap().parse().unwrap();
                let val = segs.next().unwrap().parse().unwrap();
                costs.push(cost);
                vals.push(val);
                n -= 1;
                if n == 0 {
                    bw.write_fmt(format_args!("{}\n", compute(&costs, &vals, m)))
                        .unwrap();
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

fn compute(costs: &[i32], vals: &[i32], limit: usize) -> i64 {
    let mut dp = vec![0; limit + 1];
    for (i, &cost) in costs.iter().enumerate() {
        for j in cost as usize..=limit {
            dp[j] = dp[j].max(dp[j - cost as usize] as i64 + vals[i] as i64)
        }
    }
    dp[limit]
}
