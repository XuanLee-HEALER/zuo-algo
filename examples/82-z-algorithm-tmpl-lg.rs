use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let a: String = buf.trim().into();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let b = buf.trim();
    let r = compute(&a, b);
    bw.write_fmt(format_args!("{}\n{}\n", r.0, r.1)).unwrap();
    bw.flush().unwrap();
}

fn compute(a: &str, b: &str) -> (usize, usize) {
    let z = z(b);
    let e = e(a, b, &z);
    (xor(&z), xor(&e))
}

fn xor(u: &[usize]) -> usize {
    u.iter()
        .enumerate()
        .fold(0, |acc, (i, &v)| acc ^ ((i + 1) * (v + 1)))
}

fn z(s: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let n = s.len();
    let mut z = vec![n; n];
    let (mut c, mut r) = (1, 1);
    let mut i = 1;
    while i < n {
        let mut len = if r > i { z[i - c].min(r - i) } else { 0 };
        while i + len < n && s[i + len] == s[len] {
            len += 1
        }
        if i + len > r {
            r = i + len;
            c = i
        }
        z[i] = len;
        i += 1;
    }
    z
}

fn e(s1: &str, s2: &str, s2z: &[usize]) -> Vec<usize> {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let n = s1.len();
    let mut e = vec![0; n];
    let (mut c, mut r) = (0, 0);
    let mut i = 0;
    while i < n {
        let mut len = if r > i { s2z[i - c].min(r - i) } else { 0 };
        while i + len < n && s1[i + len] == s2[len] {
            len += 1
        }
        if i + len > r {
            r = i + len;
            c = i
        }
        e[i] = len;
        i += 1;
    }
    e
}
