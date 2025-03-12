use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MAXN: i64 = 1_000_000_000_000_001;

fn main() {
    let mut fab = Vec::new();
    let mut prepre = 1;
    let mut pre = 2;
    fab.push(prepre);
    fab.push(pre);
    while pre < MAXN {
        fab.push(prepre + pre);
        (prepre, pre) = (pre, pre + prepre);
    }
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: i64 = buf.trim().parse().unwrap();
    let mut rem = n;
    let mut res = -1;
    while rem != 1 && rem != 2 {
        if let Some(v) = bs(&fab, rem) {
            if v == rem {
                res = v;
                break;
            } else {
                rem -= v;
            }
        }
    }
    bw.write_fmt(format_args!("{}\n", if res == -1 { rem } else { res }))
        .unwrap();
    bw.flush().unwrap();
}

fn bs(fab: &[i64], num: i64) -> Option<i64> {
    let mut res = None;
    let (mut l, mut r) = (0, fab.len());
    while l <= r {
        let mid = l + ((r - l) >> 1);
        match fab[mid].cmp(&num) {
            std::cmp::Ordering::Greater => {
                if mid > 0 {
                    r = mid - 1;
                } else {
                    break;
                }
            }
            std::cmp::Ordering::Less => {
                res = Some(fab[mid]);
                l = mid + 1;
            }
            std::cmp::Ordering::Equal => {
                res = Some(num);
                break;
            }
        }
    }
    res
}
