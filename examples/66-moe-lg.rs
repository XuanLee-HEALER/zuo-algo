use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            let a = segs.next().unwrap().as_bytes();
            let b = segs.next().unwrap().as_bytes();
            bw.write_fmt(format_args!("{}\n", compute(a, b))).unwrap();
            break;
        } else {
            break;
        }
    }
    bw.flush().unwrap();
}

const MOD: i32 = 1_000_000_007;

fn compute(a: &[u8], b: &[u8]) -> i32 {
    let mid = (moe(b) - moe(a) + MOD) % MOD;
    if check(a) {
        (mid + 1) % MOD
    } else {
        mid
    }
}

fn check(num: &[u8]) -> bool {
    let mut prepre = -2;
    let mut pre = -1;
    for &b in num {
        let cur = (b - b'0') as i32;
        if cur == pre || cur == prepre {
            return true;
        }
        prepre = pre;
        pre = cur;
    }
    return false;
}

// 0~num上萌数的个数
fn moe(num: &[u8]) -> i32 {
    let mut num_l: i64 = 1;
    let mut t = 1_i64;
    for b in num.iter().rev().map(|b| (*b - b'0') as i64) {
        num_l = (num_l + ((b * t) % MOD as i64)) % MOD as i64;
        t = (t * 10) % MOD as i64;
    }
    (num_l as i32 - non_moe(num) + MOD) % MOD
}

fn non_moe(num: &[u8]) -> i32 {
    if num[0] == b'0' {
        1
    } else {
        let mut dp = vec![vec![vec![vec![-1; 2]; 11]; 11]; num.len()];
        f(num, 0, 10, 10, 0, &mut dp)
    }
}

fn f(
    num: &[u8],
    len: usize,
    prepre: usize,
    pre: usize,
    free: usize,
    dp: &mut [Vec<Vec<Vec<i32>>>],
) -> i32 {
    if len == num.len() {
        1
    } else if dp[len][prepre][pre][free] != -1 {
        dp[len][prepre][pre][free]
    } else {
        let mut res = 0;
        let cur = (num[len] - b'0') as i32;
        if free == 0 {
            if pre == 10 {
                // 当前是首位
                res = (res + f(num, len + 1, 10, 10, 1, dp)) % MOD;
                for i in 1..cur {
                    res = (res + f(num, len + 1, 10, i as usize, 1, dp)) % MOD
                }
                res = (res + f(num, len + 1, 10, cur as usize, 0, dp)) % MOD
            } else {
                // 之前选了数字，且这个数字不能乱选
                for i in 0..cur {
                    if i != pre as i32 && i != prepre as i32 {
                        res = (res + f(num, len + 1, pre, i as usize, 1, dp)) % MOD
                    }
                }
                // ⚠️需要条件判断
                if cur != pre as i32 && cur != prepre as i32 {
                    res = (res + f(num, len + 1, pre, cur as usize, 0, dp)) % MOD
                }
            }
        } else {
            if pre == 10 {
                // 跳过了之前的位
                res = (res + f(num, len + 1, 10, 10, 1, dp)) % MOD;
                for i in 1..=9 {
                    res = (res + f(num, len + 1, 10, i as usize, 1, dp)) % MOD
                }
            } else {
                for i in 0..=9 {
                    if i != pre as i32 && i != prepre as i32 {
                        res = (res + f(num, len + 1, pre, i as usize, 1, dp)) % MOD
                    }
                }
            }
        }
        dp[len][prepre][pre][free] = res;
        res
    }
}
