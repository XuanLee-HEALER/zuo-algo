use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                head = true
            } else {
                let heights: Vec<i32> = buf.trim().split(" ").map(|s| s.parse().unwrap()).collect();
                bw.write_fmt(format_args!("{}\n", smallest_energy(&heights)))
                    .unwrap();
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn smallest_energy(heights: &[i32]) -> i32 {
    let mut l = 0;
    let mut r = *heights.iter().max().unwrap();
    let limit = r;
    let complete = |heights: &[i32], mut init: i32| -> bool {
        for &h in heights {
            if init >= h {
                init += init - h;
            } else {
                init -= h - init;
            }
            if init >= limit {
                return true;
            }
            if init < 0 {
                return false;
            }
        }
        true
    };
    let mut ans = 0;
    while l <= r {
        let m = l + ((r - l) >> 2);
        if complete(heights, m) {
            ans = m;
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    ans
}
