use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    buf.clear();
    for _ in 0..n {
        br.read_line(&mut buf).unwrap();
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let ori = buf
            .trim()
            .split(" ")
            .map(|v| v.parse::<i32>().unwrap())
            .reduce(|a, b| a ^ b)
            .unwrap();
        buf.clear();
        bw.write_fmt(format_args!("{}\n", if ori == 0 { "No" } else { "Yes" }))
            .unwrap();
    }
    bw.flush().unwrap();
}
