use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut cap = vec![];
    let mut money = vec![];
    let mut counter = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                head = true
            } else {
                let mut segs = buf.trim().split(" ");
                let c = segs.next().unwrap().parse().unwrap();
                let m = segs.next().unwrap().parse().unwrap();
                cap.push(c);
                money.push(m);
                counter += 1;
                if counter == n {
                    bw.write_fmt(format_args!("{}\n", compute2(n, &cap, &money)))
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

// 运行时间 414ms 占用内存 2372KB
// 能力范围很小，金钱范围很大
fn compute1(n: usize, cap: &[i32], money: &[i32]) -> i32 {
    let all_cap = cap.iter().sum::<i32>();
    // 默认可以通关，并且花多少钱，能得到的能力都是0
    let mut dp = vec![i32::MAX; all_cap as usize + 1];
    dp[0] = 0;
    for i in 0..n {
        for j in (0..=all_cap).rev() {
            if j >= cap[i] {
                if dp[(j - cap[i]) as usize] != i32::MAX {
                    dp[j as usize] = dp[j as usize].min(dp[(j - cap[i]) as usize] + money[i])
                }
            } else {
                dp[j as usize] = i32::MAX
            }
        }
    }

    let mut res = i32::MAX;
    for &c in &dp {
        res = res.min(c)
    }
    res
}

// 运行时间 16ms 占用内存 436KB
// 能力范围很大，金钱范围很小
fn compute2(n: usize, cap: &[i32], money: &[i32]) -> i32 {
    let all_money = money.iter().sum::<i32>();
    // 默认可以通关，并且花多少钱，能得到的能力都是0
    let mut dp = vec![0; all_money as usize + 1];
    for i in 0..n {
        for j in (0..=all_money).rev() {
            if j >= money[i] && dp[(j - money[i]) as usize] != i32::MIN {
                dp[j as usize] = dp[j as usize].max(dp[(j - money[i]) as usize] + cap[i])
            } else if dp[j as usize] < cap[i] {
                dp[j as usize] = i32::MIN;
            }
        }
    }
    for (i, &c) in dp.iter().enumerate() {
        if c > i32::MIN {
            return i as i32;
        }
    }
    -1
}
