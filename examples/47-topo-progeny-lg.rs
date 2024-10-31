use std::io::{self, BufRead, BufReader, BufWriter, Write};

// 每行是第i个人的后代
// 打印后代在后，前辈指向后辈
fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut n = 0;
    let mut head = false;
    let mut xhead = Vec::new();
    let mut next = Vec::new();
    let mut to = Vec::new();
    let mut p = 1;
    let mut cnt = 1;
    let mut indegree = Vec::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                xhead = vec![0; n + 1];
                next = vec![0; 50_000];
                to = vec![0; 50_000];
                indegree = vec![0; n + 1];
                head = true;
            } else {
                buf.trim().split(" ").for_each(|s| {
                    let sp: usize = s.parse().unwrap();
                    if sp != 0 {
                        // p -> sp
                        next[cnt] = xhead[p];
                        xhead[p] = cnt;
                        to[cnt] = sp;
                        cnt += 1;
                        indegree[sp] += 1;
                    }
                });
                n -= 1;
                p += 1;

                if n == 0 {
                    let mut ans = vec![0; p - 1];
                    topo(p - 1, &xhead, &next, &to, &mut indegree, &mut ans);
                    for &v in ans.iter().take(p - 2) {
                        print!("{} ", v);
                    }
                    println!("{}", ans[p - 2]);
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

fn topo(
    n: usize,
    head: &[usize],
    next: &[usize],
    to: &[usize],
    indegree: &mut [usize],
    ans: &mut [i32],
) {
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
}
