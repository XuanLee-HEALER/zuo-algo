use std::io::{self, BufRead, BufReader, BufWriter, Write};

const ARR_LEN: usize = 1_001;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut m = 0;
    let mut i = 0;
    let mut arr = [(0, 0, 0); ARR_LEN];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                m = segs.next().unwrap().parse().unwrap();
                n = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let cost = segs.next().unwrap().parse().unwrap();
                let val = segs.next().unwrap().parse().unwrap();
                let group = segs.next().unwrap().parse().unwrap();
                arr[i] = (cost, val, group);
                i += 1;
                n -= 1;

                if n == 0 {
                    bw.write_fmt(format_args!("{}\n", compute(&mut arr[..i], m)))
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

fn compute(arr: &mut [(i32, i32, i32)], limit: usize) -> i32 {
    arr.sort_by_key(|f| f.2);
    let mut dp = vec![0; limit + 1];
    let mut start = 0;
    let mut end = start + 1;
    let n = arr.len();
    while end <= n {
        while end < n {
            if arr[end].2 == arr[start].2 {
                end += 1;
            } else {
                break;
            }
        }

        for j in (0..=limit).rev() {
            for k in start..end {
                if j as i32 >= arr[k].0 {
                    dp[j] = dp[j].max(dp[(j as i32 - arr[k].0) as usize] + arr[k].1)
                }
            }
        }

        start = end;
        end += 1;
    }

    dp[limit]
}
