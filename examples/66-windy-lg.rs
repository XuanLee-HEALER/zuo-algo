use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            let a = segs.next().unwrap().parse().unwrap();
            let b = segs.next().unwrap().parse().unwrap();
            bw.write_fmt(format_args!("{}\n", compute(a, b))).unwrap();
            break;
        } else {
            break;
        }
    }
    bw.flush().unwrap();
}

fn compute(a: i32, b: i32) -> i32 {
    windy(b) - windy(a - 1)
}

fn windy(num: i32) -> i32 {
    if num <= 0 {
        1
    } else {
        let mut t = num;
        let mut len = 1;
        let mut offset = 1;
        t /= 10;
        while t > 0 {
            len += 1;
            offset *= 10;
            t /= 10;
        }
        let mut dp = vec![vec![vec![-1; 2]; 11]; len + 1];
        f(num, len, offset, 10, 0, &mut dp)
    }
}

fn f(num: i32, len: usize, offset: i32, pre: usize, free: usize, dp: &mut [Vec<Vec<i32>>]) -> i32 {
    if len == 0 {
        1
    } else if dp[len][pre][free] != -1 {
        dp[len][pre][free]
    } else {
        let mut res = 0;
        let cur = (num / offset) % 10;
        if free == 0 {
            if pre == 10 {
                res += f(num, len - 1, offset / 10, 10, 1, dp);
                for i in 1..cur {
                    res += f(num, len - 1, offset / 10, i as usize, 1, dp)
                }
                res += f(num, len - 1, offset / 10, cur as usize, 0, dp)
            } else {
                for i in 0..=9 {
                    if i <= pre as i32 - 2 || i >= pre as i32 + 2 {
                        if i < cur {
                            res += f(num, len - 1, offset / 10, i as usize, 1, dp)
                        } else if i == cur {
                            res += f(num, len - 1, offset / 10, i as usize, 0, dp)
                        }
                    }
                }
            }
        } else {
            if pre == 10 {
                res += f(num, len - 1, offset / 10, 10, 1, dp);
                for i in 1..=9 {
                    res += f(num, len - 1, offset / 10, i, 1, dp);
                }
            } else {
                for i in 0..=9 {
                    if i <= pre as i32 - 2 || i >= pre as i32 + 2 {
                        res += f(num, len - 1, offset / 10, i as usize, 1, dp)
                    }
                }
            }
        }
        dp[len][pre][free] = res;
        res
    }
}
