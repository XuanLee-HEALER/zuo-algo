use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    const MAXN: usize = 5_001;
    const MAXM: usize = 4_000_001;
    let mut n = 0;
    let mut m = 0;
    let mut idx = 0;
    let mut union_find = (0..=MAXN).collect::<Vec<usize>>();
    let mut edges = vec![(0, 0, 0); MAXM];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                let weight: usize = segs.next().unwrap().parse().unwrap();
                edges[idx] = (p1, p2, weight);
                idx += 1;
                m -= 1;

                if m == 0 {
                    let mut total_w = 0;
                    let mut total_edge = 0;
                    edges[..idx].sort_unstable_by_key(|(_, _, w)| *w);
                    for &(p1, p2, w) in &edges[..idx] {
                        if union(&mut union_find, p1, p2) {
                            total_w += w;
                            total_edge += 1;
                            if total_edge >= n - 1 {
                                break;
                            }
                        }
                    }

                    bw.write_fmt(format_args!(
                        "{}\n",
                        if total_edge == n - 1 {
                            total_w.to_string()
                        } else {
                            "orz".into()
                        }
                    ))
                    .unwrap();
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
