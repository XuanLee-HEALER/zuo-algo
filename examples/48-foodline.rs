use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MOD: usize = 80_112_002;
const MAXN: usize = 5_001;
const MAXM: usize = 500_001;

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();

    let mut n = 0;
    let mut m = 0;
    let mut xhead = vec![0; MAXN];
    let mut next = vec![0; MAXM];
    let mut to = vec![0; MAXM];
    let mut cnt = 1;
    let mut indegree = vec![0; MAXN];

    let mut head = false;

    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");

            if !head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                next[cnt] = xhead[p1];
                xhead[p1] = cnt;
                to[cnt] = p2;
                indegree[p2] += 1;
                cnt += 1;
                m -= 1;

                if m == 0 {
                    println!("{}", lines(n, &xhead, &next, &to, &mut indegree));
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

fn lines(n: usize, head: &[usize], next: &[usize], to: &[usize], indegree: &mut [usize]) -> usize {
    let mut ans = 0;
    let mut q = vec![0; n];
    let mut l = 0;
    let mut r = 0;
    let mut line = vec![0; n + 1];

    for (i, &e) in indegree.iter().skip(1).take(n).enumerate() {
        if e == 0 {
            q[r] = i + 1;
            r += 1;
            line[i + 1] = 1;
        }
    }

    while r > l {
        let cur_idx = q[l];
        if head[cur_idx] == 0 {
            ans = (line[cur_idx] + ans) % MOD;
        } else {
            let mut j = head[cur_idx];
            while j != 0 {
                if indegree[to[j]] == 1 {
                    q[r] = to[j];
                    r += 1;
                }
                indegree[to[j]] -= 1;
                line[to[j]] = (line[to[j]] + line[cur_idx]) % MOD;
                j = next[j];
            }
        }
        l += 1;
    }

    ans
}
