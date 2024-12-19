use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut limits = 0;
    let mut n = 0;
    let mut costs: Vec<i32> = vec![];
    let mut values: Vec<i32> = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                limits = segs.next().unwrap().parse().unwrap();
                n = segs.next().unwrap().parse().unwrap();
                costs = Vec::with_capacity(n as usize);
                values = Vec::with_capacity(n as usize);
                head = true;
            } else {
                let cost = segs.next().unwrap().parse().unwrap();
                let value = segs.next().unwrap().parse().unwrap();
                costs.push(cost);
                values.push(value);
                n -= 1;

                if n == 0 {
                    bw.write_fmt(format_args!("{}\n", compute(&costs, &values, limits)))
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

fn compute(costs: &[i32], values: &[i32], limits: usize) -> i32 {
    let n = costs.len();
    let mut dp = vec![0; limits + 1];
    for i in 1..=n {
        for j in (0..=limits).rev() {
            dp[j] = dp[j].max(if j as i32 >= costs[i - 1] {
                dp[j - costs[i - 1] as usize] + values[i - 1]
            } else {
                0
            })
        }
    }
    dp[limits]
}
