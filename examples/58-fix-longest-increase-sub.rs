use std::{
    i32,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

const ARR_LEN: usize = 100_001;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut n = 0;
    let mut k = 0;
    let mut arr = [0; ARR_LEN];
    let mut head = false;
    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                k = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                segs.enumerate()
                    .for_each(|(i, v)| arr[i] = v.parse().unwrap());
                if k >= n {
                    bw.write_fmt(format_args!("{}\n", n)).unwrap();
                } else {
                    bw.write_fmt(format_args!("{}\n", solve(&arr, n, k)))
                        .unwrap();
                }

                break;
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn solve(nums: &[i32], n: usize, k: usize) -> i32 {
    let nums = &nums[..n];
    let mut aid = vec![nums[n - 1]; n];
    let mut l = 1;
    let mut dp = vec![1; n];
    for i in (0..n - 1).rev() {
        if let Some(idx) = bs(&aid, l, nums[i], false) {
            aid[idx] = nums[i];
            dp[i] = idx + 1;
        } else {
            aid[l] = nums[i];
            dp[i] = l;
            l += 1;
        }
    }

    let mut res = 0;
    l = 0;
    for i in k..n {
        let left = bs(&aid, l, nums[i], true).unwrap_or(l);
        res = res.max(left + k + dp[i]);
        if let Some(idx) = bs(&aid, l, nums[i - k], true) {
            aid[idx] = nums[i - k];
        } else {
            aid[l] = nums[i - k];
            l += 1;
        }
    }
    res = res.max(l + k);

    res as i32
}

fn bs(nums: &[i32], len: usize, val: i32, more: bool) -> Option<usize> {
    let mut res = None;
    if len > 0 {
        let (mut l, mut r) = (0, len - 1);

        while l <= r {
            let m = l + ((r - l) >> 1);
            if more {
                if nums[m] > val {
                    res = Some(m);
                    if m > 0 {
                        r = m - 1;
                    } else {
                        break;
                    }
                } else {
                    l = m + 1;
                }
            } else if nums[m] < val {
                res = Some(m);
                if m > 0 {
                    r = m - 1;
                } else {
                    break;
                }
            } else {
                l = m + 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod test_solve {
    use rand::{thread_rng, Rng};

    use crate::solve;

    #[test]
    fn test_solve() {
        let times = 20_000;
        let mut rng = thread_rng();
        for _ in 0..times {
            let cur_len = rng.gen_range(1..=10_000);
            let cur_k = rng.gen_range(1..=cur_len);
            let mut arr = Vec::with_capacity(cur_len);
            for i in 0..cur_len {
                arr.push(rng.gen_range(1..=1_000_000));
            }
            // print!("cur_len {} cur_k {} ", cur_len, cur_k);
            // println!(" res is {}", solve(&arr, cur_len, cur_k));
        }
    }
}
