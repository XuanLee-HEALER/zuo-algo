use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut n = 0;
    let mut m = 0;
    let mut cnt = 0;
    let mut head = false;
    let mut union_find = vec![];
    let mut edges = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                union_find = (0..=n).collect::<Vec<usize>>();
                edges = vec![(0, 0, 0); m * 2 + 1];
                head = true;
            } else {
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                let w: usize = segs.next().unwrap().parse().unwrap();
                edges[cnt] = (p1, p2, w);
                edges[cnt + 1] = (p2, p1, w);
                cnt += 2;
                m -= 1;

                if m == 0 {
                    let mut ans = 0;
                    let mut cnt = 0;
                    edges.sort_unstable_by_key(|e| e.2);
                    for &(p1, p2, w) in &edges {
                        if union(&mut union_find, p1, p2) {
                            ans = ans.max(w);
                            cnt += 1;
                            if cnt == n - 1 {
                                break;
                            }
                        }
                    }
                    bw.write_fmt(format_args!("{} {}\n", cnt, ans)).unwrap();
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

fn find(arr: &mut [usize], v: usize) -> usize {
    if arr[v] != v {
        arr[v] = find(arr, arr[v]);
    }
    arr[v]
}

fn union(arr: &mut [usize], v1: usize, v2: usize) -> bool {
    let b_v1 = find(arr, v1);
    let b_v2 = find(arr, v2);
    if b_v1 == b_v2 {
        false
    } else {
        arr[b_v1] = b_v2;
        true
    }
}
