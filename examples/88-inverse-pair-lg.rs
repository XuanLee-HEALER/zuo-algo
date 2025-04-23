use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let src = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();
    // bw.write_fmt(format_args!("{}\n", compute1(n, src)))
    //     .unwrap();
    bw.write_fmt(format_args!("{}\n", compute2(n, src)))
        .unwrap();
    bw.flush().unwrap()
}

// 归并分治
fn compute1(n: usize, arr: Vec<i32>) -> usize {
    let mut aid = vec![0; n];
    let mut arr = arr;
    f(&mut arr, 0, n - 1, &mut aid)
}

fn f(arr: &mut [i32], l: usize, r: usize, aid: &mut [i32]) -> usize {
    if l == r {
        0
    } else {
        let m = l + ((r - l) >> 1);
        f(arr, l, m, aid) + f(arr, m + 1, r, aid) + merge(arr, l, m, r, aid)
    }
}

fn merge(arr: &mut [i32], l: usize, m: usize, r: usize, aid: &mut [i32]) -> usize {
    let mut res = 0;
    let mut r1 = r;
    for l1 in (l..=m).rev() {
        while r1 > m && arr[l1] <= arr[r1] {
            r1 -= 1;
        }
        if r1 <= m {
            break;
        } else {
            res += r1 - m
        }
    }

    let (mut l1, mut r1) = (l, m + 1);
    let mut ct = 0;
    while l1 <= m && r1 <= r {
        if arr[l1] < arr[r1] {
            aid[ct] = arr[l1];
            l1 += 1;
        } else {
            aid[ct] = arr[r1];
            r1 += 1;
        }
        ct += 1;
    }
    while l1 <= m {
        aid[ct] = arr[l1];
        l1 += 1;
        ct += 1;
    }
    while r1 <= r {
        aid[ct] = arr[r1];
        r1 += 1;
        ct += 1;
    }
    arr[l..=r].copy_from_slice(&mut aid[..ct]);
    res
}

// 树状数组
fn compute2(n: usize, arr: Vec<i32>) -> usize {
    let d_arr = discrate_array(&arr);
    let mut arr = arr;
    let mut max = 0;
    for e in &mut arr {
        *e = (d_arr[1..].binary_search(e).unwrap() + 1) as i32;
        max = max.max(*e)
    }
    let mut idx_tree = vec![0; max as usize + 1];
    let mut res = 0;
    for &v in arr.iter().rev() {
        res += sum(&idx_tree, v as usize - 1);
        add(&mut idx_tree, v as usize)
    }
    res
}

fn bk(i: i32) -> i32 {
    i & -i
}

fn add(arr: &mut [i32], i: usize) {
    let mut i = i as i32;
    while i < arr.len() as i32 {
        arr[i as usize] += 1;
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

#[cfg(test)]
mod tests {
    use crate::discrate_array;

    #[test]
    fn test_discrate_array() {
        let ori = vec![1, 2, 2, 3, 3, 3];
        let d_arr = discrate_array(&ori);
        assert_eq!(d_arr, vec![0, 1, 2, 3]);
        let ori = vec![1, 2, 3, 4];
        let d_arr = discrate_array(&ori);
        assert_eq!(d_arr, vec![0, 1, 2, 3, 4]);
        let ori = vec![0, -3, -2, -1, 3, 2, 1, -3];
        let d_arr = discrate_array(&ori);
        assert_eq!(d_arr, vec![0, -3, -2, -1, 0, 1, 2, 3]);
    }
}
