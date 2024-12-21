use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MAX_NUMS: usize = 100_000;
fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut w = 0;
    let group_unit = usize::next_power_of_two(MAX_NUMS);
    let mut costs = vec![];
    let mut vals = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                w = segs.next().unwrap().parse().unwrap();
                costs = Vec::with_capacity(n * group_unit);
                vals = Vec::with_capacity(n * group_unit);
                head = true;
            } else {
                let val: i32 = segs.next().unwrap().parse().unwrap();
                let cost: i32 = segs.next().unwrap().parse().unwrap();
                let mut num: i32 = segs.next().unwrap().parse().unwrap();
                let mut k = 1;
                while k <= num {
                    costs.push(k * cost);
                    vals.push(k * val);
                    num -= k;
                    k <<= 1;
                }
                if num > 0 {
                    costs.push(num * cost);
                    vals.push(num * val);
                }
                n -= 1;

                if n == 0 {
                    bw.write_fmt(format_args!("{}\n", compute(&costs, &vals, w)))
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

fn compute(costs: &[i32], vals: &[i32], limit: usize) -> i32 {
    let mut dp = vec![0; limit + 1];
    for (i, &cost) in costs.iter().enumerate() {
        for j in (0..=limit).rev() {
            dp[j] = dp[j].max(if j as i32 >= cost {
                dp[(j as i32 - cost) as usize] + vals[i]
            } else {
                0
            })
        }
    }
    dp[limit]
}

// 不优化，TLE
// fn compute(costs: &[i32], vals: &[i32], nums: &[i32], limit: usize) -> i32 {
//     let mut dp = vec![0; limit + 1];
//     for (i, &cost) in costs.iter().enumerate() {
//         for j in (0..=limit).rev() {
//             for k in 1..=nums[i] {
//                 dp[j] = dp[j].max(if j as i32 >= cost * k {
//                     dp[(j as i32 - cost * k) as usize] + k * vals[i]
//                 } else {
//                     0
//                 })
//             }
//         }
//     }
//     dp[limit]
// }
