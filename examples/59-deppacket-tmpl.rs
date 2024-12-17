use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MAX_N: usize = 61;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut count = 0;
    let mut n = 0;
    let mut m = 0;
    let mut costs = Vec::with_capacity(MAX_N);
    let mut vals = Vec::with_capacity(MAX_N);
    let mut kings = Vec::with_capacity(MAX_N);
    let mut fans = vec![0; MAX_N];
    let mut fellows = vec![vec![]; MAX_N];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                m = segs.next().unwrap().parse().unwrap();
                n = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let cost = segs.next().unwrap().parse().unwrap();
                let content = segs.next().unwrap().parse::<i32>().unwrap();
                let associate = segs.next().unwrap().parse::<i32>().unwrap();
                n -= 1;
                costs.push(cost);
                vals.push(cost * content);
                if associate == 0 {
                    kings.push(true);
                } else {
                    kings.push(false);
                    fans[associate as usize - 1] += 1;
                    fellows[associate as usize - 1].push(count);
                }
                count += 1;

                if n == 0 {
                    bw.write_fmt(format_args!(
                        "{}\n",
                        compute(&costs, &vals, &kings, &fans, &fellows, m)
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

fn compute(
    costs: &[i32],
    vals: &[i32],
    kings: &[bool],
    fans: &[i32],
    fellows: &[Vec<i32>],
    n: usize,
) -> i32 {
    let mut dp = vec![0; n + 1];
    for (i, &cost) in costs.iter().enumerate() {
        if kings[i] {
            for j in (0..=n).rev() {
                if j as i32 >= cost {
                    dp[j] = dp[j].max(dp[(j as i32 - cost) as usize] + vals[i])
                }
                // ⚠️这里只要有超过1个附件都要判断第一个，而不是仅等于1才判断
                if fans[i] >= 1 && j as i32 >= cost + costs[fellows[i][0] as usize] {
                    dp[j] = dp[j].max(
                        dp[(j as i32 - cost - costs[fellows[i][0] as usize]) as usize]
                            + vals[i]
                            + vals[fellows[i][0] as usize],
                    )
                }
                if fans[i] == 2 {
                    if j as i32 >= cost + costs[fellows[i][1] as usize] {
                        dp[j] = dp[j].max(
                            dp[(j as i32 - cost - costs[fellows[i][1] as usize]) as usize]
                                + vals[i]
                                + vals[fellows[i][1] as usize],
                        )
                    }
                    if j as i32
                        >= cost + costs[fellows[i][0] as usize] + costs[fellows[i][1] as usize]
                    {
                        dp[j] = dp[j].max(
                            dp[(j as i32
                                - cost
                                - costs[fellows[i][0] as usize]
                                - costs[fellows[i][1] as usize])
                                as usize]
                                + vals[i]
                                + vals[fellows[i][0] as usize]
                                + vals[fellows[i][1] as usize],
                        )
                    }
                }
            }
        }
    }
    dp[n]
}
