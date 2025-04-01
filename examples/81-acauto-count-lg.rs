use std::{
    collections::VecDeque,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

const MOD: i32 = 1e9 as i32 + 7;
const CHAR_LEN: usize = 10;
const MAXN: usize = 1501;

fn u8_to_usize(u: u8) -> usize {
    (u - b'0') as usize
}

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: Vec<u8> = buf.trim().into();
    buf.clear();
    let mut ac = AC::new(MAXN);
    br.read_line(&mut buf).unwrap();
    let m: usize = buf.trim().parse().unwrap();
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        ac.insert(buf.trim());
    }
    ac.set_fail();
    let nn = n.len();
    let mut dp = vec![vec![vec![vec![-1; 2]; 2]; MAXN]; nn];
    bw.write_fmt(format_args!(
        "{}\n",
        compute_1(&n, &ac, n.len(), 0, 0, 0, 0, &mut dp)
    ))
    .unwrap();

    bw.flush().unwrap();
}

fn compute_1(
    num: &[u8],
    ac: &AC,
    n: usize,
    i: usize,
    j: usize,
    free: usize,
    has: usize,
    dp: &mut [Vec<Vec<Vec<i32>>>],
) -> i32 {
    // ⚠️要先判断是否匹配到了目标字符串
    if ac.alert[j] {
        0
    } else if i == n {
        has as i32
    } else if dp[i][j][free][has] != -1 {
        dp[i][j][free][has]
    } else {
        let mut res = 0;
        let limit = dp[i][j][free][has] = res;
        res
    }
}

fn compute(
    num: &[u8],
    ac: &AC,
    n: usize,
    i: usize,
    j: usize,
    free: usize,
    has: usize,
    dp: &mut [Vec<Vec<Vec<i32>>>],
) -> i32 {
    // ⚠️要先判断是否匹配到了目标字符串
    if ac.alert[j] {
        0
    } else if i == n {
        has as i32
    } else if dp[i][j][free][has] != -1 {
        dp[i][j][free][has]
    } else {
        let mut res = 0;
        match (free, has) {
            (0, 0) => {
                res = ((res as i64 + compute(num, ac, n, i + 1, 0, 1, 0, dp) as i64) % MOD as i64)
                    as i32;
                for ti in b'1'..num[i] {
                    res = ((res as i64
                        + compute(num, ac, n, i + 1, ac.tree[j][u8_to_usize(ti)], 1, 1, dp) as i64)
                        % MOD as i64) as i32;
                }
                res = ((res as i64
                    + compute(num, ac, n, i + 1, ac.tree[j][u8_to_usize(num[i])], 0, 1, dp) as i64)
                    % MOD as i64) as i32;
            }
            (0, 1) => {
                for ti in b'0'..num[i] {
                    res = ((res as i64
                        + compute(num, ac, n, i + 1, ac.tree[j][u8_to_usize(ti)], 1, 1, dp) as i64)
                        % MOD as i64) as i32;
                }
                res = ((res as i64
                    + compute(num, ac, n, i + 1, ac.tree[j][u8_to_usize(num[i])], 0, 1, dp) as i64)
                    % MOD as i64) as i32;
            }
            (1, 0) => {
                res = ((res as i64 + compute(num, ac, n, i + 1, 0, 1, 0, dp) as i64) % MOD as i64)
                    as i32;
                for ti in b'1'..=b'9' {
                    res = ((res as i64
                        + compute(num, ac, n, i + 1, ac.tree[j][u8_to_usize(ti)], 1, 1, dp) as i64)
                        % MOD as i64) as i32;
                }
            }
            (1, 1) => {
                for ti in b'0'..=b'9' {
                    res = ((res as i64
                        + compute(num, ac, n, i + 1, ac.tree[j][u8_to_usize(ti)], 1, 1, dp) as i64)
                        % MOD as i64) as i32;
                }
            }
            _ => unreachable!(),
        }
        dp[i][j][free][has] = res;
        res
    }
}

struct AC {
    ct: usize,
    tree: Vec<[usize; CHAR_LEN]>,
    fail: Vec<usize>,
    alert: Vec<bool>,
}

impl AC {
    fn new(n: usize) -> Self {
        Self {
            ct: 0,
            tree: vec![[0; CHAR_LEN]; n],
            fail: vec![0; n],
            alert: vec![false; n],
        }
    }

    fn insert(&mut self, s: &str) {
        let mut cur = 0;
        for &b in s.as_bytes() {
            let idx = u8_to_usize(b);
            if self.tree[cur][idx] == 0 {
                self.ct += 1;
                self.tree[cur][idx] = self.ct
            }
            cur = self.tree[cur][idx]
        }
        self.alert[cur] = true
    }

    fn set_fail(&mut self) {
        let mut q = VecDeque::new();
        for u in self.tree[0] {
            if u != 0 {
                q.push_back(u);
            }
        }
        while let Some(p) = q.pop_front() {
            for b in 0..CHAR_LEN {
                if self.tree[p][b] == 0 {
                    self.tree[p][b] = self.tree[self.fail[p]][b]
                } else {
                    self.fail[self.tree[p][b]] = self.tree[self.fail[p]][b];
                    q.push_back(self.tree[p][b]);
                }
            }
            self.alert[p] |= self.alert[self.fail[p]]
        }
    }
}
