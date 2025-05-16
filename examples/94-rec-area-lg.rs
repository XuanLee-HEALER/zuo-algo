use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut events = Vec::with_capacity(n << 1);
    let mut ysort = Vec::with_capacity(n << 1 | 1);
    for _ in 0..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let x1: i64 = segs.next().unwrap().parse().unwrap();
        let y1: i64 = segs.next().unwrap().parse().unwrap();
        let x2: i64 = segs.next().unwrap().parse().unwrap();
        let y2: i64 = segs.next().unwrap().parse().unwrap();
        events.push((x1, y1, y2, 1));
        events.push((x2, y1, y2, -1));
        ysort.push(y1);
        ysort.push(y2);
    }
    events.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    ysort.sort_unstable();
    ysort.dedup();
    ysort.push(*ysort.last().unwrap());
    let m = ysort.len();
    let mut st = ST::new(m - 1);
    st.build(&ysort, 1, m - 1, 1);
    let mut pre = 0;
    let mut res = 0;
    for (x, y1, y2, v) in events {
        res += st.q() * (x - pre);
        pre = x;
        st.add(
            ysort.binary_search(&y1).unwrap() + 1,
            ysort.binary_search(&y2).unwrap(),
            v,
            1,
            m - 1,
            1,
        );
    }
    bw.write_fmt(format_args!("{}\n", res)).unwrap();
    bw.flush().unwrap()
}

struct ST {
    len: Vec<i64>,
    cover: Vec<i64>,
    times: Vec<i32>,
}

impl ST {
    fn new(n: usize) -> Self {
        return Self {
            len: vec![0; n << 2],
            cover: vec![0; n << 2],
            times: vec![0; n << 2],
        };
    }

    fn build(&mut self, ysort: &[i64], l: usize, r: usize, i: usize) {
        if l < r {
            let m = l + ((r - l) >> 1);
            self.build(ysort, l, m, i << 1);
            self.build(ysort, m + 1, r, i << 1 | 1)
        }
        self.len[i] = ysort[r] - ysort[l - 1]
    }

    fn up(&mut self, i: usize) {
        if self.times[i] == 0 {
            self.cover[i] = if i << 1 < self.cover.len() {
                self.cover[i << 1] + self.cover[i << 1 | 1]
            } else {
                0
            }
        } else {
            self.cover[i] = self.len[i]
        }
    }

    fn add(&mut self, jobl: usize, jobr: usize, jobv: i32, l: usize, r: usize, i: usize) {
        if jobl <= l && jobr >= r {
            self.times[i] += jobv;
        } else {
            let m = l + ((r - l) >> 1);
            if jobl <= m {
                self.add(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.add(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
        }
        self.up(i);
    }

    fn q(&self) -> i64 {
        self.cover[1]
    }
}
