use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MAXN: usize = 1_000_001;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut counter = 0;
    let mut lasts = vec![-1; MAXN];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                head = true
            } else {
                let mut segs = buf.trim().split(" ");
                let beg = segs.next().unwrap().parse().unwrap();
                let end: usize = segs.next().unwrap().parse().unwrap();
                lasts[end as usize] = lasts[end as usize].max(beg);
                counter += 1;
                if counter == n {
                    bw.write_fmt(format_args!("{}\n", compute(&lasts))).unwrap();
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

fn compute(lasts: &[i32]) -> i32 {
    let mut pre = 0;
    let mut res = 0;
    for i in 0..MAXN {
        if lasts[i] >= pre {
            res += 1;
            pre = i as i32;
        }
    }
    res
}
