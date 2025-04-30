use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut tasks = Vec::with_capacity(m);
    let mut posts = Vec::with_capacity(m << 1 | 1);
    for i in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let l: usize = segs.next().unwrap().parse().unwrap();
        let r: usize = segs.next().unwrap().parse().unwrap();
        tasks.push((i + 1, l, r));
        posts.push(l);
        posts.push(r);
    }
    posts.push(n);
    posts.sort_unstable();
    posts.dedup();
    let tn = posts.len();
    let mut rank = Vec::with_capacity(tn << 1);
    rank.push(*posts.first().unwrap());
    for &post in posts.iter().skip(1) {
        if post - rank.last().unwrap() > 1 {
            rank.push(post - 1);
            rank.push(post);
        } else {
            rank.push(post);
        }
    }
    tasks.iter_mut().for_each(|v| {
        *v = (
            v.0,
            rank.binary_search(&v.1).unwrap() + 1,
            rank.binary_search(&v.2).unwrap() + 1,
        )
    });
    let tn = rank.len();
    let mut st = STAddSum::new(tn);
    for (task_id, task_l, task_r) in tasks {
        st.set(task_l, task_r, task_id, 1, tn, 1);
    }
    let mut hs = HashSet::new();
    bw.write_fmt(format_args!(
        "{}\n",
        st.query(1, rank.binary_search(&n).unwrap() + 1, 1, tn, 1, &mut hs)
    ))
    .unwrap();
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STAddSum {
    info: Vec<usize>,
}

impl STAddSum {
    fn new(n: usize) -> Self {
        Self {
            info: vec![0; n << 2],
        }
    }

    fn down(&mut self, i: usize) {
        self.info[i << 1] = self.info[i];
        self.info[i << 1 | 1] = self.info[i];
        self.info[i] = 0;
    }

    fn set(&mut self, jobl: usize, jobr: usize, jobv: usize, l: usize, r: usize, i: usize) {
        if jobl <= l && jobr >= r {
            self.info[i] = jobv
        } else {
            let m = m(l, r);
            if self.info[i] > 0 {
                self.down(i);
            }
            if jobl <= m {
                self.set(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.set(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
        }
    }

    fn query(
        &mut self,
        jobl: usize,
        jobr: usize,
        l: usize,
        r: usize,
        i: usize,
        hs: &mut HashSet<usize>,
    ) -> usize {
        if l == r {
            if self.info[i] > 0 && !hs.contains(&self.info[i]) {
                hs.insert(self.info[i]);
                1
            } else {
                0
            }
        } else {
            let m = m(l, r);
            if self.info[i] > 0 {
                self.down(i);
            }
            let mut res = 0;
            if jobl <= m {
                res += self.query(jobl, jobr, l, m, i << 1, hs);
            }
            if jobr > m {
                res += self.query(jobl, jobr, m + 1, r, i << 1 | 1, hs);
            }
            res
        }
    }
}
