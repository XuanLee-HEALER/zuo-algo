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
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let all_stones = buf
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i32>>();
        bw.write_fmt(format_args!(
            "{}\n",
            if compute(&all_stones) { "YES" } else { "NO" }
        ))
        .unwrap();
        buf.clear();
    }
    bw.flush().unwrap();
}

fn low_zero(mut v: i32) -> i32 {
    let mut res = 0;
    while v > 0 {
        if v & 1 == 0 {
            break;
        }
        v >>= 1;
        res += 1
    }
    res
}

fn compute(stones: &[i32]) -> bool {
    let mut res = 0;
    let mut a = 0;
    let mut ct = 0;
    for &stone in stones {
        if ct != 0 && ct % 2 == 1 {
            res = res ^ low_zero((a - 1) | (stone - 1))
        }
        ct += 1;
        a = stone;
    }
    res != 0
}

fn dumb(a: i32, b: i32) -> i32 {
    let max = a.max(b);
    let mut dp = vec![vec![-1; max as usize + 1]; max as usize + 1];
    d(a, b, &mut dp)
}

fn d(a: i32, b: i32, dp: &mut [Vec<i32>]) -> i32 {
    if a == 1 && b == 1 {
        0
    } else if dp[a as usize][b as usize] != -1 {
        dp[a as usize][b as usize]
    } else {
        let mut appear = vec![false; (a.max(b)) as usize];
        for i in 1..b {
            dp[i as usize][(b - i) as usize] = d(i, b - i, dp);
            appear[dp[i as usize][(b - i) as usize] as usize] = true
        }
        for i in 1..a {
            dp[(a - i) as usize][i as usize] = d(a - i, i, dp);
            appear[dp[(a - i) as usize][i as usize] as usize] = true
        }
        for (ti, &b) in appear.iter().enumerate() {
            if !b {
                dp[a as usize][b as usize] = ti as i32;
                return ti as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test_solution {
    use rand::{Rng, thread_rng};

    #[test]
    fn test_sg() {
        println!("   0 1 2 3 4 5 6 7 8");
        for i in 1..=9 {
            print!("{}> ", i - 1);
            for j in 1..=9 {
                if j >= i {
                    print!("{} ", super::dumb(i, j))
                } else {
                    print!("x ");
                }
            }
            println!()
        }

        println!("   0 1 2 3 4 5 6 7 8");
        for i in 1..=9 {
            print!("{}> ", i - 1);
            for j in 1..=9 {
                if j >= i {
                    print!("{} ", super::dumb(i, j))
                } else {
                    print!("x ");
                }
            }
            println!()
        }
    }
}
