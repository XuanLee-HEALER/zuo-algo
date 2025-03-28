use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let s1: String = buf.trim().into();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let s2 = buf.trim();
    bw.write_fmt(format_args!("{}\n", compute(&s1, s2)))
        .unwrap();
    bw.flush().unwrap();
}

fn compute(s1: &str, s2: &str) -> String {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let n = s1.len();
    let m = s2.len();
    let nxt = next(s2);
    let mut stack = vec![(0, 0); n];
    let mut size = 0;
    let mut i = 0;
    let mut j = 0;
    while i < n {
        if s1[i] == s2[j] {
            // 如果两个字符相同，插入它们匹配的索引位置
            stack[size] = (i as i32, j as i32);
            i += 1;
            j += 1;
            size += 1;
        } else if j == 0 {
            stack[size] = (i as i32, -1);
            i += 1;
            size += 1;
        } else {
            j = nxt[j] as usize;
        }
        if j == m {
            size -= m;
            j = if size > 0 {
                (stack[size - 1].1 + 1) as usize
            } else {
                0
            }
        }
    }
    return String::from_utf8(
        stack[..size]
            .into_iter()
            .map(|v| s1[v.0 as usize])
            .collect(),
    )
    .unwrap();
}

fn next(bs: &[u8]) -> Vec<i32> {
    let n = bs.len();
    let mut nxt = vec![-1; n];
    if n == 1 {
        return nxt;
    }
    nxt[1] = 0;
    let mut cn = 0;
    let mut i = 2;
    while i < n {
        if bs[i - 1] == bs[cn as usize] {
            cn += 1;
            nxt[i] = cn;
            i += 1;
        } else if cn > 0 {
            cn = nxt[cn as usize]
        } else {
            nxt[i] = cn;
            i += 1;
        }
    }
    nxt
}
