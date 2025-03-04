use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut cap = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                cap = Vec::with_capacity(n);
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                let ai: i32 = segs.next().unwrap().parse().unwrap();
                let bi: i32 = segs.next().unwrap().parse().unwrap();
                cap.push((ai, bi));
                n -= 1;
                if n == 0 {
                    bw.write_fmt(format_args!("{:.1}\n", compute(&mut cap)))
                        .unwrap();
                    break;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn compute(arr: &mut [(i32, i32)]) -> f64 {
    arr.sort_unstable_by(|a, b| (a.0 - a.1).abs().cmp(&(b.0 - b.1).abs()));
    let mut max_a = arr[0].0;
    let mut max_b = arr[0].1;
    let mut res = 0;
    for &(ai, bi) in arr.iter().skip(1) {
        if ai > bi {
            res = res.max(bi + max_b)
        } else {
            res = res.max(ai + max_a)
        }
        max_a = max_a.max(ai);
        max_b = max_b.max(bi);
    }
    res as f64 / 2.0
}
