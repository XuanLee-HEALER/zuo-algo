use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut cnt = vec![0; n];
    let hasher = StrHasher::new(131);
    for i in 0..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        cnt[i] = hasher.hash(buf.trim());
    }
    cnt.sort_unstable();
    let mut res = 1;
    cnt.windows(2).for_each(|sub| {
        if sub[0] != sub[1] {
            res += 1
        }
    });
    bw.write_fmt(format_args!("{}\n", res)).unwrap();
    bw.flush().unwrap();
}

struct StrHasher {
    code: [u64; 62],
    base: u64,
}

impl StrHasher {
    fn new(base: u64) -> Self {
        let mut code = [0_u64; 62];
        for (i, v) in code.iter_mut().enumerate() {
            *v = (i + 1) as u64
        }
        Self { code, base }
    }

    fn hash(&self, s: &str) -> u64 {
        let mut res = 0_u64;
        for &b in s.as_bytes() {
            match b {
                b if b >= b'0' && b <= b'9' => {
                    res = res
                        .wrapping_mul(self.base)
                        .wrapping_add(self.code[(b - b'0') as usize])
                }
                b if b >= b'a' && b <= b'z' => {
                    res = res
                        .wrapping_mul(self.base)
                        .wrapping_add(self.code[(b - b'a' + 10) as usize])
                }
                b if b >= b'A' && b <= b'Z' => {
                    res = res
                        .wrapping_mul(self.base)
                        .wrapping_add(self.code[(b - b'A' + 36) as usize])
                }
                _ => panic!(),
            }
        }
        res
    }
}
