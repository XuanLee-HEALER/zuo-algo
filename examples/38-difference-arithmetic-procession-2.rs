use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut m: usize = 0;
    const LENGTH: usize = 1_000_001;
    const OFFSET: usize = 30_001;
    let mut arr = vec![0; OFFSET * 2 + LENGTH];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let segs = buf.trim().split(" ").take(2).collect::<Vec<&str>>();
                n = segs[0].parse().unwrap();
                m = segs[1].parse().unwrap();
                head = true;
            } else if n > 0 {
                let segs = buf.trim().split(" ").take(2).collect::<Vec<&str>>();
                let v = segs[0].parse().unwrap();
                let loc: usize = segs[1].parse().unwrap();
                fall(&mut arr, v, loc + OFFSET);
                n -= 1;

                if n == 0 {
                    build(&mut arr);
                    for v in &arr[(1 + OFFSET)..(OFFSET + m)] {
                        bw.write_fmt(format_args!("{} ", *v)).unwrap();
                    }
                    bw.write_fmt(format_args!("{}", arr[OFFSET + m])).unwrap();
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

/// 如果l==r，无法计算公差，需要直接提供
fn set(diff_arr: &mut [i32], l: usize, r: usize, beg: i32, end: i32, d: i32) {
    diff_arr[l] += beg;
    diff_arr[l + 1] += d - beg;
    diff_arr[r + 1] -= d + end;
    diff_arr[r + 2] += end;
}

fn fall(diff_arr: &mut [i32], v: i32, loc: usize) {
    set(
        diff_arr,
        loc - 3 * v as usize + 1,
        loc - 2 * v as usize,
        1,
        v,
        1,
    );
    set(diff_arr, loc - 2 * v as usize + 1, loc, v - 1, -v, -1);
    set(diff_arr, loc + 1, loc + 2 * v as usize, -v + 1, v, 1);
    set(
        diff_arr,
        loc + 2 * v as usize + 1,
        loc + 3 * v as usize - 1,
        v - 1,
        1,
        -1,
    );
}

fn build(diff_arr: &mut [i32]) {
    let mut sum = 0;
    for v in diff_arr.iter_mut() {
        sum += *v;
        *v = sum;
    }

    sum = 0;
    for v in diff_arr.iter_mut() {
        sum += *v;
        *v = sum;
    }
}
