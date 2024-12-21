use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut w = 0;
    let mut details = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                w = segs.next().unwrap().parse().unwrap();
                details = Vec::with_capacity(n + 1);
                head = true;
            } else {
                let val: i32 = segs.next().unwrap().parse().unwrap();
                let cost: i32 = segs.next().unwrap().parse().unwrap();
                let num: i32 = segs.next().unwrap().parse().unwrap();
                details.push((cost, val, num));
                n -= 1;

                if n == 0 {
                    bw.write_fmt(format_args!("{}\n", compute2(&details, w)))
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

fn val1(details: &[(i32, i32, i32)], dp: &[Vec<i32>], i: usize, j: usize) -> i32 {
    dp[i][j] - j as i32 / details[i].0 * details[i].1
}

fn compute1(details: &[(i32, i32, i32)], limit: usize) -> i32 {
    let n = details.len();
    let mut dp = vec![vec![0; limit + 1]; n + 1];
    let mut q = vec![0; limit + 1];
    for (i, &(cost, val, num)) in details.iter().enumerate() {
        let mut x_mod = 0;
        while x_mod < cost.min(limit as i32) {
            let (mut l, mut r) = (0, 0);
            let (mut start, mut end) = (x_mod, x_mod);
            while end <= limit as i32 {
                let cur_val = val1(details, &dp, i, end as usize);
                while r > l && q[r - 1] < cur_val {
                    r -= 1;
                }
                q[r] = cur_val;
                r += 1;

                if end - start >= cost * (num + 1) {
                    if val1(details, &dp, i, start as usize) == q[l] {
                        l += 1
                    };
                    start += cost;
                }

                dp[i + 1][end as usize] = q[l] + end / cost * val;
                end += cost;
            }
            x_mod += 1;
        }
    }
    dp[n][limit]
}

fn val2(dp: &[i32], cost: i32, val: i32, j: usize) -> i32 {
    dp[j] - j as i32 / cost * val
}

// 空间压缩
fn compute2(details: &[(i32, i32, i32)], limit: usize) -> i32 {
    let mut dp = vec![0; limit + 1];
    let mut q = vec![0; limit + 1];
    for &(cost, val, num) in details {
        let (mut l, mut r) = (0, 0);
        let mut x_mod = 0;
        while x_mod < cost.min(limit as i32) {
            let mut k = limit as i32 - x_mod;
            let end = k - (cost * (num - 1));
            while k >= 0.max(end) {
                while r > l && val2(&dp, cost, val, q[r - 1]) <= val2(&dp, cost, val, k as usize) {
                    r -= 1;
                }
                q[r] = k as usize;
                r += 1;
                k -= cost;
            }

            let mut j = limit as i32 - x_mod;
            while j >= 0 {
                if k >= 0 {
                    while r > l
                        && val2(&dp, cost, val, q[r - 1]) <= val2(&dp, cost, val, k as usize)
                    {
                        r -= 1;
                    }
                    q[r] = k as usize;
                    r += 1;
                }

                dp[j as usize] = val2(&dp, cost, val, q[l]) + j / cost * val;

                if q[l] == j as usize {
                    l += 1;
                }
                j -= cost;
                k -= cost;
            }
            x_mod += 1;
        }
    }
    dp[limit]
}
