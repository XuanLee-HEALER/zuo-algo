use std::io::{self, BufRead, BufReader, BufWriter, Write};

// 运行时间 54ms 占用内存 8960KB
fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut n = 0;
    let mut m = 0;
    let mut head = false;
    let mut xhead = Vec::new();
    let mut next = Vec::new();
    let mut to = Vec::new();
    let mut cnt = 1;
    let mut indegree = Vec::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                xhead = vec![0; n + 1];
                next = vec![0; m + 1];
                to = vec![0; m + 1];
                indegree = vec![0; n + 1];
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                next[cnt] = xhead[p1];
                xhead[p1] = cnt;
                to[cnt] = p2;
                cnt += 1;
                indegree[p2] += 1;
                m -= 1;

                if m == 0 {
                    let mut ans = vec![0; n];
                    if topo(n, &xhead, &next, &to, &mut indegree, &mut ans) {
                        for &v in ans.iter().take(n - 1) {
                            print!("{} ", v);
                        }
                        println!("{}", ans[n - 1]);
                    } else {
                        println!("-1");
                    }
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn topo(
    n: usize,
    head: &[usize],
    next: &[usize],
    to: &[usize],
    indegree: &mut [usize],
    ans: &mut [i32],
) -> bool {
    let mut q = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    let mut cnt = 0;

    for (idx, &idg) in indegree.iter().skip(1).enumerate() {
        if idg == 0 {
            q[r] = idx + 1;
            r += 1;
        }
    }

    while r > l {
        ans[cnt] = q[l] as i32;
        let mut j = head[q[l]];
        while j != 0 {
            if indegree[to[j]] == 1 {
                q[r] = to[j];
                r += 1;
            }
            indegree[to[j]] -= 1;
            j = next[j];
        }
        l += 1;
        cnt += 1;
    }

    n == cnt
}
