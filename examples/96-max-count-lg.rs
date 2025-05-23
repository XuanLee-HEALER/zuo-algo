use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let ori = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();
    let (bucket, buck_count, left, right) = bucket(n, &ori);
    let st = ST::new(&buck_count);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let l: usize = segs.next().unwrap().parse().unwrap();
        let r: usize = segs.next().unwrap().parse().unwrap();
        let a = right[bucket[l]] - l + 1;
        let b = r - left[bucket[r]] + 1;
        let rr = if bucket[l] + 1 < bucket[r] {
            let c = st.q_max(bucket[l] + 1, bucket[r] - 1);
            a.max(b).max(c)
        } else if bucket[l] == bucket[r] {
            r - l + 1
        } else {
            a.max(b)
        };
        bw.write_fmt(format_args!("{}\n", rr)).unwrap()
    }
    br.read_line(&mut buf).unwrap();
    bw.flush().unwrap()
}

fn bucket(n: usize, arr: &[i32]) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    // bucket[i]是原数组i位置的属于哪个桶
    let mut bucket = vec![1; n + 1];
    let mut beg = 1;
    let mut i = 1;
    let mut buck_size = 1;
    while i < arr.len() {
        if arr[i] > arr[i - 1] {
            buck_size += 1;
            beg += 1;
        }
        bucket[i + 1] = beg;
        i += 1;
    }
    let mut buck_info = vec![0; buck_size + 1];
    let mut left_info = vec![0; buck_size + 1];
    let mut right_info = vec![0; buck_size + 1];
    let mut pre = 0;
    for (i, &buck_id) in bucket.iter().enumerate().skip(1) {
        buck_info[buck_id] += 1;
        if pre == 0 {
            left_info[buck_id] = i;
            pre = buck_id
        } else if buck_id > pre {
            left_info[buck_id] = i;
            right_info[buck_id - 1] = i - 1;
            pre = buck_id
        }
    }
    right_info[buck_size] = n;
    (bucket, buck_info, left_info, right_info)
}

struct ST {
    log2: Vec<i32>,
    max_st: Vec<Vec<usize>>,
}

fn least_pow2(n: usize) -> usize {
    let mut r = 0;
    while 1 << r <= n >> 1 {
        r += 1
    }
    r
}

impl ST {
    fn new(ori: &[usize]) -> Self {
        let n = ori.len() - 1;
        let m = least_pow2(n);
        let mut log2 = vec![-1; n + 1];
        let mut max_st = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            log2[i] = log2[i >> 1] + 1;
            max_st[i][0] = ori[i];
        }
        for j in 1..=m {
            for i in 1..=(n - (1 << j) + 1) {
                max_st[i][j] = max_st[i][j - 1].max(max_st[i + (1 << (j - 1))][j - 1]);
            }
        }
        Self { log2, max_st }
    }

    fn q_max(&self, l: usize, r: usize) -> usize {
        let d = r - l + 1;
        let w = self.log2[d] as usize;
        self.max_st[l][w].max(self.max_st[r - (1 << w) + 1][w])
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    fn count(arr: &[i32], l: usize, r: usize) -> usize {
        let l = l - 1;
        let r = r - 1;
        let mut pre = arr[0] - 1;
        let mut ct = 0;
        let mut max = 0;
        for &v in &arr[l..=r] {
            if v != pre {
                if ct != 0 {
                    max = max.max(ct);
                }
                ct = 1;
                pre = v;
            } else {
                ct += 1
            }
        }
        max = max.max(ct);
        max
    }

    #[test]
    fn test_max_count() {
        let mut rng = thread_rng();
        for i in 1..=1000 {
            let mut ori = Vec::with_capacity(i);
            for _ in 0..i {
                ori.push(rng.gen_range(-100_000..=100_000));
            }
            ori.sort_unstable();
            let (bucket, buck_count, left, right) = super::bucket(i, &ori);
            let st = super::ST::new(&buck_count);
            for _ in 0..50000 {
                let l = rng.gen_range(1..=i);
                let r = rng.gen_range(l..=i);
                let a = right[bucket[l]] - l + 1;
                let b = r - left[bucket[r]] + 1;
                let rr = if bucket[l] + 1 < bucket[r] {
                    let c = st.q_max(bucket[l] + 1, bucket[r] - 1);
                    a.max(b).max(c)
                } else if bucket[l] == bucket[r] {
                    r - l + 1
                } else {
                    a.max(b)
                };
                assert_eq!(rr, count(&ori, l, r), "ori: {:?}, l: {l}, r: {r}", ori)
            }
        }
    }
}
