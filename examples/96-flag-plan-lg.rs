use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut lines = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let beg: usize = segs.next().unwrap().parse().unwrap();
        let mut end: usize = segs.next().unwrap().parse().unwrap();
        if end < beg {
            end += m;
        }
        lines.push((beg, end));
    }
    compute(n, m, lines)
        .iter()
        .for_each(|v| bw.write_fmt(format_args!("{} ", v)).unwrap());
    bw.write_fmt(format_args!("\n")).unwrap();
    bw.flush().unwrap()
}

fn compute(n: usize, m: usize, lines: Vec<(usize, usize)>) -> Vec<usize> {
    let mut lines_with_idx: Vec<(usize, usize, usize)> = lines
        .iter()
        .enumerate()
        .map(|(i, (beg, end))| (*beg, *end, i + 1))
        .collect();
    lines_with_idx.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    for i in 0..lines_with_idx.len() {
        let (beg, end, loc) = lines_with_idx[i];
        lines_with_idx.push((beg + m, end + m, loc));
    }
    let mm = least_pow2(n);
    let e = n << 1;
    // 如果st表中的值是0，表示没有线段可走
    let mut st = vec![vec![0; mm + 1]; e + 1];
    // 一条线段至少可以一步走到自己
    let mut j = 1;
    for i in 1..=e {
        while j + 1 <= e && lines_with_idx[j].0 <= lines_with_idx[i - 1].1 {
            j += 1
        }
        // 第i条线段一步能到的最远的其它线段
        st[i][0] = j;
    }
    for j in 1..=mm {
        for i in 1..=e {
            st[i][j] = st[st[i][j - 1]][j - 1]
        }
    }
    let mut res = vec![0; n];
    for (ti, &(beg, _, i)) in lines_with_idx[0..n].iter().enumerate() {
        let aim = beg + m;
        let mut r = 0;
        let mut cur = ti + 1;
        for mm in (0..=mm).rev() {
            let next = st[cur][mm];
            // 使用 < 表示离得最近但是没到
            if next != 0 && lines_with_idx[next - 1].1 < aim {
                r += 1 << mm;
                cur = next;
            }
        }
        // 带上本身，以及最后需要加1步
        res[i - 1] = r + 2
    }
    res
}

fn least_pow2(v: usize) -> usize {
    let mut res = 0;
    while 1 << res <= v >> 1 {
        res += 1;
    }
    res as usize
}
