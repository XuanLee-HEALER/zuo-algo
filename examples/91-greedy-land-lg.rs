use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut st = STAddMax::new(n);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let op: usize = segs.next().unwrap().parse().unwrap();
        let r1: usize = segs.next().unwrap().parse().unwrap();
        let r2: usize = segs.next().unwrap().parse().unwrap();
        if op == 1 {
            st.put_beg(r1, 1, n, 1);
            st.put_end(r2, 1, n, 1);
        } else {
            let start_num = STAddMax::query(&st.beg_pos, 1, r2, 1, n, 1);
            let end_num = if r1 > 1 {
                STAddMax::query(&st.end_pos, 1, r1 - 1, 1, n, 1)
            } else {
                0
            };
            bw.write_fmt(format_args!("{}\n", start_num - end_num))
                .unwrap()
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STAddMax {
    beg_pos: Vec<usize>,
    end_pos: Vec<usize>,
}

impl STAddMax {
    fn new(n: usize) -> Self {
        Self {
            beg_pos: vec![0; n << 2],
            end_pos: vec![0; n << 2],
        }
    }

    fn up(info: &mut [usize], i: usize) {
        info[i] = info[i << 1] + info[i << 1 | 1];
    }

    fn put_val(info: &mut [usize], jobi: usize, l: usize, r: usize, i: usize) {
        if l == r {
            info[i] += 1;
        } else {
            let m = m(l, r);
            if jobi <= m {
                Self::put_val(info, jobi, l, m, i << 1);
            }
            if jobi > m {
                Self::put_val(info, jobi, m + 1, r, i << 1 | 1);
            }
            Self::up(info, i);
        }
    }

    fn put_beg(&mut self, jobi: usize, l: usize, r: usize, i: usize) {
        Self::put_val(&mut self.beg_pos, jobi, l, r, i);
    }

    fn put_end(&mut self, jobi: usize, l: usize, r: usize, i: usize) {
        Self::put_val(&mut self.end_pos, jobi, l, r, i);
    }

    fn query(info: &[usize], jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> usize {
        if jobl <= l && jobr >= r {
            info[i]
        } else {
            let m = m(l, r);
            let mut res = 0;
            if jobl <= m {
                res += Self::query(info, jobl, jobr, l, m, i << 1);
            }
            if jobr > m {
                res += Self::query(info, jobl, jobr, m + 1, r, i << 1 | 1);
            }
            res
        }
    }
}
