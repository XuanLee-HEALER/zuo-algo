use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let arr = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();
    bw.write_fmt(format_args!("{}\n", compute(n, arr))).unwrap();
    bw.flush().unwrap()
}

fn compute(n: usize, arr: Vec<i32>) -> usize {
    let mut arr = arr;
    let d_arr = discrate_array(&arr);
    arr.iter_mut()
        .for_each(|v| *v = (d_arr[1..].binary_search(v).unwrap() + 1) as i32);
    let mut uniple = vec![0; n + 1];
    let mut douple = vec![0; n + 1];
    let mut res = 0;
    for &v in &arr {
        res += sum(&douple, v as usize - 1);
        add(&mut douple, v as usize, sum(&uniple, v as usize - 1) as i32);
        add(&mut uniple, v as usize, 1);
    }
    res
}

fn bk(i: i32) -> i32 {
    i & -i
}

fn add(arr: &mut [i32], i: usize, v: i32) {
    let mut i = i as i32;
    while i < arr.len() as i32 {
        arr[i as usize] += v;
        i += bk(i);
    }
}

fn sum(arr: &[i32], i: usize) -> usize {
    let mut res = 0;
    let mut i = i as i32;
    while i > 0 {
        res += arr[i as usize] as usize;
        i -= bk(i);
    }
    res
}

fn discrate_array(ori: &[i32]) -> Vec<i32> {
    let n = ori.len();
    let mut res = vec![0; n + 1];
    res[1..].copy_from_slice(&ori);
    res[1..].sort_unstable();
    let mut cur = 2;
    let mut nxt = cur;
    while nxt <= n {
        if res[nxt] != res[nxt - 1] {
            res[cur] = res[nxt];
            cur += 1;
        }
        nxt += 1;
    }
    res[..cur].into()
}
