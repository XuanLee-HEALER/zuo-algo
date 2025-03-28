use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let l: i32 = buf.trim().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    bw.write_fmt(format_args!("{}\n", l - last_next(buf.trim())))
        .unwrap();
    bw.flush().unwrap();
}

fn last_next(s: &str) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut nxt = vec![-1; n + 1];
    if n == 1 {
        return 0;
    }
    nxt[1] = 0;
    let mut i = 2;
    let mut cn = 0;
    while i <= n {
        if s[i - 1] == s[cn as usize] {
            cn += 1;
            nxt[i] = cn;
            i += 1
        } else if cn == 0 {
            nxt[i] = cn;
            i += 1;
        } else {
            cn = nxt[cn as usize]
        }
    }
    nxt[n]
}
