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
            .collect::<Vec<i32>>();
        buf.clear();
        bw.write_fmt(format_args!(
            "{}\n",
            if compute(&ori) { "John" } else { "Brother" }
        ))
        .unwrap();
    }
    bw.flush().unwrap();
}

fn compute(ori: &[i32]) -> bool {
    let mut ex = 0;
    let mut sum = 0;
    for &v in ori {
        ex ^= v;
        sum += v;
    }
    if sum == ori.len() as i32 {
        if sum & 1 == 1 { false } else { true }
    } else {
        if ex == 0 { false } else { true }
    }
}
