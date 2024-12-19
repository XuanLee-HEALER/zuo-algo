use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut h = 0;
    let mut extra = 0;
    let mut costs = vec![];
    let mut vals = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                h = segs.next().unwrap().parse().unwrap();
                costs = Vec::with_capacity(n);
                vals = Vec::with_capacity(n);
                head = true;
            } else {
                let cost = segs.next().unwrap().parse().unwrap();
                let val = segs.next().unwrap().parse().unwrap();
                costs.push(cost);
                vals.push(val);
                extra = extra.max(val);
                n -= 1;

                if n == 0 {
                    bw.write_fmt(format_args!(
                        "{}\n",
                        compute(&costs, &vals, h, extra as usize + h)
                    ))
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

fn compute(costs: &[i32], vals: &[i32], ori_limit: usize, limit: usize) -> i32 {
    let mut dp = vec![i32::MAX; limit + 1];
    dp[0] = 0;
    for (i, &cost) in costs.iter().enumerate() {
        for j in 0..=limit {
            dp[j] = dp[j].min(
                if j as i32 >= cost && dp[(j as i32 - cost) as usize] != i32::MAX {
                    dp[(j as i32 - cost) as usize] + vals[i]
                } else {
                    i32::MAX
                },
            )
        }
    }

    let mut res = dp[ori_limit];
    for &v in dp.iter().skip(ori_limit + 1) {
        res = res.min(v);
    }
    res
}
