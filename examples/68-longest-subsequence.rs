use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let first_arr = buf
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let second_arr = buf
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    bw.write_fmt(format_args!("{}\n", compute(n, &first_arr, &second_arr)))
        .unwrap();
    bw.flush().unwrap();
}

fn compute(n: usize, f_arr: &[i32], s_arr: &[i32]) -> i32 {
    let mut idx_arr = vec![0; n + 1];
    for (i, &e) in f_arr.iter().enumerate() {
        idx_arr[e as usize] = i
    }
    let mut c_arr = vec![0; n];
    for (i, &e) in s_arr.iter().enumerate() {
        c_arr[i] = idx_arr[e as usize]
    }
    let mut res = 0;
    let mut aid = vec![];
    for &e in &c_arr {
        res = res.max(if let Some(j) = bs_max_left(&aid, e) {
            aid[j] = e;
            j + 1
        } else {
            aid.push(e);
            aid.len()
        })
    }
    res as i32
}

fn bs_max_left(arr: &[usize], num: usize) -> Option<usize> {
    let mut res = None;
    if arr.is_empty() {
        res
    } else {
        let mut beg = 0;
        let mut end = arr.len() - 1;
        while beg <= end {
            let mid = beg + ((end - beg) >> 1);
            if arr[mid] >= num {
                res = Some(mid);
                if mid > 0 {
                    end = mid - 1;
                } else {
                    break;
                }
            } else {
                beg = mid + 1;
            }
        }
        res
    }
}
