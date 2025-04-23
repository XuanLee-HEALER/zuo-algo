use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let mut max_c = 0;
    let ori = buf
        .trim()
        .split_whitespace()
        .map(|v| {
            let v = v.parse().unwrap();
            max_c = max_c.max(v);
            v
        })
        .collect::<Vec<usize>>();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let m: usize = buf.trim().parse().unwrap();
    let mut tasks = Vec::with_capacity(m);
    let mut id = 0;
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let l: usize = segs.next().unwrap().parse().unwrap();
        let r: usize = segs.next().unwrap().parse().unwrap();
        tasks.push((l, r, id));
        id += 1;
    }
    tasks.sort_unstable_by(|v1, v2| v1.1.cmp(&v2.1));
    let mut mp = vec![0; max_c + 1];
    let mut itree = vec![0; n + 1];
    let mut res = vec![0; m];
    let mut c = 1;
    for &(l, r, id) in &tasks {
        while c <= r {
            if mp[ori[c - 1]] == 0 {
                // 如果这个颜色没有出现过，更新它
                mp[ori[c - 1]] = c;
                add(&mut itree, n, c, 1);
            } else {
                // 更新位置
                add(&mut itree, n, mp[ori[c - 1]], -1);
                add(&mut itree, n, c, 1);
                mp[ori[c - 1]] = c;
            }
            c += 1;
        }
        res[id] = range(&itree, l, r)
    }
    for &v in &res {
        bw.write_fmt(format_args!("{}\n", v)).unwrap()
    }
    bw.flush().unwrap()
}

fn bk(i: i32) -> i32 {
    i & -i
}

fn add(arr: &mut [i32], n: usize, i: usize, v: i32) {
    let mut i = i as i32;
    while i <= n as i32 {
        arr[i as usize] += v;
        i += bk(i)
    }
}

fn sum(arr: &[i32], i: usize) -> usize {
    let mut i = i as i32;
    let mut res = 0;
    while i > 0 {
        res += arr[i as usize];
        i -= bk(i)
    }
    res as usize
}

fn range(arr: &[i32], l: usize, r: usize) -> usize {
    sum(arr, r) - sum(arr, l - 1)
}
