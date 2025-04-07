use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MOD: usize = 19930726;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split(" ");
    let n = segs.next().unwrap().parse().unwrap();
    let k = segs.next().unwrap().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let s = buf.trim();
    bw.write_fmt(format_args!("{}\n", compute(n, k, s)))
        .unwrap();
    bw.flush().unwrap()
}

fn compute(n: usize, mut k: usize, s: &str) -> i64 {
    let mut ctn = if n & 1 == 1 { n + 1 } else { n };
    let mut ct = vec![0; ctn];
    let s = manacher_s(s);
    let s = s.as_bytes();
    let n = s.len();
    let mut p = vec![0; n];
    let (mut i, mut c, mut r) = (0, 0, 0);
    while i < n {
        let mut len = if r > i { p[c * 2 - i].min(r - i) } else { 1 };
        while i + len < n && i >= len && s[i + len] == s[i - len] {
            len += 1;
        }
        if i + len > r {
            r = i + len;
            c = i;
        }
        p[i] = len;
        ct[len - 1] += 1;
        i += 1;
    }
    let mut res = 1;
    let mut last = 0;
    ctn -= 1;
    loop {
        if (ct[ctn] > 0 || last > 0) && k > 0 {
            let cur = ct[ctn] + last;
            let cur_ct = cur.min(k);
            res = res * q_exp(ctn, cur_ct) % MOD;
            // ⚠️要持续累积
            last = cur;
            k -= cur_ct;
        }
        if ctn == 1 || k == 0 {
            break;
        } else {
            ctn -= 2;
        }
    }
    if k > 0 { -1 } else { res as i64 }
}

fn manacher_s(s: &str) -> String {
    let mut res = vec![b'#'; s.len() * 2 + 1];
    for (i, &b) in s.as_bytes().iter().enumerate() {
        res[2 * i + 1] = b;
    }
    String::from_utf8(res).unwrap()
}

fn q_exp(mut a: usize, mut b: usize) -> usize {
    let mut res = 1;
    while b > 0 {
        if b & 1 > 0 {
            res = res * a % MOD
        }
        b >>= 1;
        a = a * a % MOD
    }
    res
}
