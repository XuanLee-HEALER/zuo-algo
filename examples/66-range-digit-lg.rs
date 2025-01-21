use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            let a: i64 = segs.next().unwrap().parse().unwrap();
            let b: i64 = segs.next().unwrap().parse().unwrap();
            for i in 0..10 {
                bw.write_fmt(format_args!(
                    "{} ",
                    digit_occurrence(b, i) - digit_occurrence(a - 1, i)
                ))
                .unwrap();
            }
            bw.write_fmt(format_args!("\n")).unwrap();
            break;
        } else {
            break;
        }
    }
    bw.flush().unwrap();
}

// 1~num中digit出现了多少次
fn digit_occurrence(num: i64, digit: i64) -> i64 {
    let mut res = 0;
    let mut cur = num;
    let mut pre;
    let mut suf = 1;
    let mut suf_num = 0;
    while cur > 0 {
        pre = cur / 10;
        let cur_d = cur % 10;
        if digit == 0 {
            pre -= 1
        }

        if digit < cur_d {
            res += ((pre + 1) * suf) as i64
        } else if digit > cur_d {
            res += (pre * suf) as i64
        } else {
            res += (pre * suf + suf_num + 1) as i64
        }
        suf_num += cur_d * suf;
        suf = suf * 10;
        cur /= 10;
    }
    res
}
