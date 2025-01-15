use std::io::{self, BufRead, BufReader, BufWriter, Write};

const ARR_LEN: usize = 100_001;

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut k = 0;
    let mut arr = [0; ARR_LEN];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                k = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                for (i, v) in segs.enumerate() {
                    arr[i] = v.parse().unwrap();
                }
                bw.write_fmt(format_args!("{}\n", compute2(&arr[..n], k)))
                    .unwrap();
                break;
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

// 运行时间 9ms 占用内存 2316KB
fn compute2(arr: &[i32], k: i32) -> i32 {
    let n = arr.len();
    let mut minsum = vec![0; n];
    let mut minsum_end = vec![0; n];
    minsum[n - 1] = arr[n - 1];
    minsum_end[n - 1] = n - 1;
    for (i, &v) in arr.iter().enumerate().rev().skip(1) {
        minsum_end[i] = if v < v + minsum[i + 1] {
            minsum[i] = v;
            i
        } else {
            minsum[i] = v + minsum[i + 1];
            minsum_end[i + 1]
        }
    }

    let mut res = 0;
    let mut begin = 0;
    let mut end = 0;
    let mut t_sum = 0;
    while begin < n && end < n {
        if t_sum + minsum[end] <= k {
            res = res.max((minsum_end[end] - begin + 1) as i32);
            t_sum += minsum[end];
            end = minsum_end[end] + 1
        } else {
            t_sum -= arr[begin];
            begin += 1;
        }
    }

    res
}

// 运行时间 10ms 占用内存 1552KB
fn compute1(arr: &[i32], k: i32) -> i32 {
    let n = arr.len();
    let mut sum = 0;
    let mut presum = vec![0; n + 1];
    for (i, &v) in arr.iter().enumerate() {
        sum += v;
        presum[i + 1] = if sum > presum[i] { sum } else { presum[i] }
    }
    // 大于等于v最左
    let find = |idx, v| -> Option<usize> {
        let mut l = 0;
        let mut r = idx;
        let mut res = None;
        while l <= r {
            let mid = l + ((r - l) >> 1);
            if presum[mid] >= v {
                res = Some(mid);
                if mid > 0 {
                    r = mid - 1
                } else {
                    break;
                }
            } else {
                l = mid + 1
            }
        }
        res
    };
    let mut res = 0;
    let mut sum = 0;
    for (i, &v) in arr.iter().enumerate() {
        sum += v;
        if let Some(j) = find(i, sum - k) {
            res = res.max((i - j) as i32 + 1)
        }
    }
    res
}
