use std::io::{self, BufRead, BufReader, BufWriter, Write};

/// 提交十几次发现数据类型应该是i64
/// ⚠️注意！！！！！！！
fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());

    let mut buf = String::new();
    let mut problem_arr = vec![0; 10_000_005];
    let mut head: bool = false;
    let mut counter = 0;
    let mut n = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let headline: Vec<&str> = buf.trim().split(" ").collect();
                n = headline[0].parse().unwrap();
                counter = headline[1].parse().unwrap();
                head = true
            } else if counter > 0 {
                let p: Vec<&str> = buf.trim().split(" ").collect();
                let l = p[0].parse().unwrap();
                let r = p[1].parse().unwrap();
                let s = p[2].parse().unwrap();
                let e = p[3].parse().unwrap();
                set(&mut problem_arr, l, r, s, e, (e - s) / (r - l) as i64);
                counter -= 1;

                if counter == 0 {
                    let ans = difference_arithmetic_procession_mode(&mut problem_arr, n);
                    bw.write_fmt(format_args!("{} {}\n", ans.0, ans.1)).unwrap();
                    break;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }

    bw.flush().unwrap();
}

fn set(diff_arr: &mut [i64], l: usize, r: usize, beg: i64, end: i64, d: i64) {
    diff_arr[l] += beg;
    diff_arr[l + 1] += d - beg;
    diff_arr[r + 1] -= d + end;
    diff_arr[r + 2] += end;
}

fn difference_arithmetic_procession_mode(dif_arr: &mut [i64], n: usize) -> (i64, i64) {
    // an = a1 + (n-1)*d
    // d  = (an - a1)/(n-1)
    // n = (end-beg)+1
    // d = (an-a1)/(end-beg)
    let mut sum = 0;
    for v in dif_arr[1..=n].iter_mut() {
        sum += *v;
        *v = sum;
    }

    sum = 0;
    for v in dif_arr[1..=n].iter_mut() {
        sum += *v;
        *v = sum;
    }

    let mut xor_ans = 0;
    let mut max = 0;
    for v in dif_arr[1..=n].iter() {
        xor_ans ^= *v;
        max = max.max(*v)
    }

    (xor_ans, max)
}
