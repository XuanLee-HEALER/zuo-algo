use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split(" ");
    let n: i32 = segs.next().unwrap().parse().unwrap();
    let p: i32 = segs.next().unwrap().parse().unwrap();
    let mut inv = vec![1; n as usize + 1];
    (1..=n).for_each(|v| {
        if v == 1 {
            bw.write(b"1\n").unwrap();
        } else {
            inv[v as usize] = p - (inv[(p % v) as usize] as i64 * (p / v) as i64 % p as i64) as i32;
            bw.write_fmt(format_args!("{}\n", inv[v as usize])).unwrap()
        }
    });

    bw.flush().unwrap();
}
