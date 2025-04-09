use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    for _ in 0..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let s: String = buf.trim().into();
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let p = buf.trim();
        bw.write_fmt(format_args!("{}\n", compute(&s, p))).unwrap();
    }
    bw.flush().unwrap()
}

fn compute(s: &str, p: &str) -> i32 {
    let hasher = SubStrHasher::new(s, p, 131);
    let n = s.len();
    let m = p.len();
    let mut i = 0;
    let mut res = 0;
    while i + m <= n {
        if !check(&hasher, i, i + m - 1, 3) {
            res += 1;
        }
        i += 1;
    }
    res
}

// l-r是否存在超过k处不同，条件是能找到第k+1处不同
fn check(hasher: &SubStrHasher, mut l: usize, r: usize, k: usize) -> bool {
    let ll = l;
    let mut dif = 0;
    for _ in 0..=k {
        if l <= r {
            if let Some(loc) = bs(hasher, ll, l, r) {
                l = loc + 2;
                if loc < r {
                    dif += 1;
                }
            } else {
                l += 1;
                dif += 1;
            }
        }
    }
    dif > k
}

// 从l到r，两个字符串最长的那个相同点的位置在哪
fn bs(hasher: &SubStrHasher, beg: usize, mut l: usize, mut r: usize) -> Option<usize> {
    let mut res = None;
    while l <= r {
        let m = l + ((r - l) >> 1);
        let h1 = hasher.hash1(l, m);
        let h2 = hasher.hash2(l - beg, m - beg);
        if h1 == h2 {
            // l-m范围，s和p完全相同，记录答案，尝试往后找
            res = Some(m);
            l = m + 1;
        } else {
            // l-m范围，s和p不一样，缩小范围
            if m > l {
                r = m - 1;
            } else {
                break;
            }
        }
    }
    res
}

struct SubStrHasher {
    exp: Vec<u64>,
    hash1: Vec<u64>,
    hash2: Vec<u64>,
}

impl SubStrHasher {
    fn new(s1: &str, s2: &str, base: u64) -> Self {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len().max(s2.len());
        let val = |u: u8| -> u64 { (u - b'A' + 11) as u64 };
        let mut exp = vec![1_u64; n];
        let mut res1 = val(s1[0]);
        let mut res2 = val(s2[0]);
        let mut hash1 = vec![res1; n];
        let mut hash2 = vec![res2; n];
        for i in 1..n {
            if i < s1.len() {
                res1 = res1.wrapping_mul(base).wrapping_add(val(s1[i]));
                hash1[i] = res1;
            }
            if i < s2.len() {
                res2 = res2.wrapping_mul(base).wrapping_add(val(s2[i]));
                hash2[i] = res2;
            }
            exp[i] = exp[i - 1].wrapping_mul(base);
        }
        Self { exp, hash1, hash2 }
    }

    fn hash1(&self, l: usize, r: usize) -> u64 {
        if l > 0 {
            self.hash1[r].wrapping_sub(self.hash1[l - 1].wrapping_mul(self.exp[r - l + 1]))
        } else {
            self.hash1[r]
        }
    }

    fn hash2(&self, l: usize, r: usize) -> u64 {
        if l > 0 {
            self.hash2[r].wrapping_sub(self.hash2[l - 1].wrapping_mul(self.exp[r - l + 1]))
        } else {
            self.hash2[r]
        }
    }
}
