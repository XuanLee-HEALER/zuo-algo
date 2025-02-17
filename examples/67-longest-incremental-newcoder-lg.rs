use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                head = true;
            } else {
                let arr = buf
                    .trim()
                    .split(" ")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i32>>();
                let mut dp = vec![0; n];
                let t_res = t(n, &arr, &mut dp);
                let mut res = vec![i32::MAX; t_res as usize];
                for (i, &v) in dp.iter().enumerate() {
                    if v == t_res {
                        res[0] = arr[i]
                    } else if arr[i] > res[(t_res - v - 1) as usize] {
                        res[(t_res - v) as usize] = arr[i]
                    }
                }
                res.iter()
                    .for_each(|s| bw.write_fmt(format_args!("{} ", s)).unwrap());
                bw.write_fmt(format_args!("\n")).unwrap();
                break;
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn t(n: usize, arr: &[i32], dp: &mut [i32]) -> i32 {
    let mut aid = vec![];
    let mut r = 0;
    for (i, &v) in arr.iter().enumerate().rev() {
        dp[i] = if let Some(j) = bs_left_min(&aid, v) {
            aid[j] = v;
            (j + 1) as i32
        } else {
            aid.push(v);
            aid.len() as i32
        };
        if dp[i] > r {
            r = dp[i]
        }
    }
    r
}

fn bs_left_min(arr: &[i32], num: i32) -> Option<usize> {
    if arr.len() > 0 {
        let mut beg = 0;
        let mut end = arr.len() - 1;
        let mut res = None;
        while beg <= end {
            let mid = beg + ((end - beg) >> 1);
            if arr[mid] <= num {
                res = Some(mid);
                if mid > 0 {
                    end = mid - 1;
                } else {
                    break;
                }
            } else {
                beg = mid + 1;
            }
        }
        res
    } else {
        None
    }
}
