use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let call_params: Vec<i32> = buf.trim().split(" ").map(|v| v.parse().unwrap()).collect();
    let (a, b, p) = (call_params[0] as i64, call_params[1] as i64, call_params[2]);
    bw.write_fmt(format_args!("{}^{} mod {}={}\n", a, b, p, exp(a, b, p)))
        .unwrap();
    bw.flush().unwrap()
}

fn exp(a: i64, mut b: i64, p: i32) -> i32 {
    let tp = p as i64;
    let mut res = 1;
    let mut x = a;
    while b > 0 {
        if b & 1 != 0 {
            res = res * x % tp;
        }
        x = x * x % tp;
        b >>= 1;
    }
    res as i32
}
