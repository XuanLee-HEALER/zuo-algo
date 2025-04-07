use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    bw.write_fmt(format_args!("{}\n", compute(buf.trim())))
        .unwrap();
    bw.flush().unwrap();
}

fn compute(s: &str) -> i32 {
    let s = manacher_s(s);
    let s = s.as_bytes();
    let n = s.len();
    let mut p = vec![0; n];
    let (mut i, mut c, mut r) = (0, 0, 0);
    while i < n {
        let mut len = if r > i { p[2 * c - i].min(r - i) } else { 1 };
        while i + len < n && i >= len && s[i + len] == s[i - len] {
            len += 1;
        }
        if i + len > r {
            r = i + len;
            c = i;
        }
        p[i] = len;
        i += 1;
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut j = 0;
    for (i, &v) in p.iter().enumerate() {
        while i + v > j {
            left[j] = j - i;
            j += 2;
        }
    }
    j = n - 1;
    for (i, &v) in p.iter().enumerate().rev() {
        while i >= v && j >= 2 && i - v < j {
            right[j] = i - j;
            j -= 2;
        }
    }
    let mut res = 0;
    let mut i = 2;
    while i <= n - 3 {
        res = res.max(left[i] + right[i]);
        i += 2;
    }
    res as i32
}

fn manacher_s(s: &str) -> String {
    let mut res = vec![b'#'; s.len() * 2 + 1];
    for (i, &b) in s.as_bytes().iter().enumerate() {
        res[2 * i + 1] = b;
    }
    String::from_utf8(res).unwrap()
}
