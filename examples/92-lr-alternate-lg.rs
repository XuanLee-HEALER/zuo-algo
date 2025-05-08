use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut st = STResetSum::new(n);
    st.build(1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let l: usize = buf.trim().parse().unwrap();
        st.reset(l, 1, n, 1);
        bw.write_fmt(format_args!("{}\n", st.len_a[1])).unwrap()
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STResetSum {
    arr: Vec<u8>,
    len_a: Vec<usize>,
    pre_a: Vec<usize>,
    suf_a: Vec<usize>,
}

impl STResetSum {
    fn new(n: usize) -> Self {
        Self {
            arr: vec![0; n + 1],
            len_a: vec![0; n << 2],
            pre_a: vec![0; n << 2],
            suf_a: vec![0; n << 2],
        }
    }

    fn build(&mut self, l: usize, r: usize, i: usize) {
        if l == r {
            self.len_a[i] = 1;
            self.pre_a[i] = 1;
            self.suf_a[i] = 1;
        } else {
            let m = m(l, r);
            self.build(l, m, i << 1);
            self.build(m + 1, r, i << 1 | 1);
            self.up(l, r, i);
        }
    }

    fn up(&mut self, l: usize, r: usize, i: usize) {
        let mut max_len_a = self.len_a[i << 1].max(self.len_a[i << 1 | 1]);
        let mut max_pre_a = self.pre_a[i << 1];
        let mut max_suf_a = self.suf_a[i << 1 | 1];
        let m = m(l, r);
        let ln = m - l + 1;
        let rn = r - m;
        if self.arr[m] != self.arr[m + 1] {
            max_len_a = max_len_a.max(self.suf_a[i << 1] + self.pre_a[i << 1 | 1]);
            if max_pre_a == ln {
                max_pre_a += self.pre_a[i << 1 | 1];
            }
            if max_suf_a == rn {
                max_suf_a += self.suf_a[i << 1];
            }
        }
        self.len_a[i] = max_len_a;
        self.pre_a[i] = max_pre_a;
        self.suf_a[i] = max_suf_a;
    }

    fn reset(&mut self, jobl: usize, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if l == r {
            self.arr[jobl] ^= 1;
        } else {
            let m = m(l, r);
            if jobl <= m {
                self.reset(jobl, l, m, i << 1);
            } else {
                self.reset(jobl, m + 1, r, i << 1 | 1);
            }
            // ⚠️子范围调整完之后要更新父范围
            self.up(l, r, i);
        }
    }
}
