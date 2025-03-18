use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    buf.clear();
    for _ in 0..n {
        br.read_line(&mut buf).unwrap();
        let n: i64 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", if is_prime(n) { "Yes" } else { "No" }))
            .unwrap();
        buf.clear();
    }
    bw.flush().unwrap();
}

const BASE_PRIME: [i64; 20] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
];

fn is_prime(n: i64) -> bool {
    if n <= 2 {
        n == 2
    } else if n % 2 == 0 {
        false
    } else {
        for v in BASE_PRIME {
            if v < n && miller_ranbin(v, n) {
                return false;
            }
        }
        true
    }
}

// 根据选择的a（小于n的质数）和被测试数n
// 返回n是否为合数
fn miller_ranbin(a: i64, n: i64) -> bool {
    let mut u = n - 1;
    let mut t = 0;
    while u & 1 == 1 {
        t += 1;
        u >>= 1;
    }
    let mut x1 = exponent(a, u, n);
    for _ in 1..=t {
        let x2 = exponent(x1, 2, n);
        if x2 == 1 && x1 != 1 && x1 != n - 1 {
            return true;
        }
        x1 = x2;
    }
    if x1 != 1 {
        return true;
    }
    false
}

fn exponent(mut x: i64, mut e: i64, r: i64) -> i64 {
    let mut res = 1;
    while e > 0 {
        if e & 1 != 0 {
            res = ((res as i128 * x as i128) % r as i128) as i64
        }
        x = ((x as i128 * x as i128) % r as i128) as i64;
        e >>= 1;
    }
    res
}
