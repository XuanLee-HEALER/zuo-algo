use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MAX_N: usize = 302;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut x_head = false;
    let mut n = 0;
    let mut m = 0;
    let mut cur = 1;
    let mut head = vec![0; MAX_N];
    let mut next = vec![0; MAX_N];
    let mut to = vec![0; MAX_N];
    let mut vals = vec![0; MAX_N];
    let mut dfn_cnt = 1;
    let mut dfn_vals = vec![0; MAX_N];
    let mut size = vec![0; MAX_N];
    let mut dp = vec![vec![0; MAX_N]; MAX_N + 1];
    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !x_head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                x_head = true;
            } else {
                let par: usize = segs.next().unwrap().parse().unwrap();
                let score = segs.next().unwrap().parse().unwrap();
                // par -> cur
                next[cur] = head[par];
                head[par] = cur as i32;
                to[cur] = cur;
                vals[cur] = score;
                cur += 1;
                if cur > n {
                    dfn(
                        &head,
                        &next,
                        &to,
                        &vals,
                        &mut dfn_vals,
                        &mut size,
                        &mut dfn_cnt,
                        0,
                    );
                    for i in (2..=(n + 1)).rev() {
                        for j in 1..=m {
                            dp[i][j] = dp[i + size[i]][j].max(dp[i + 1][j - 1] + dfn_vals[i])
                        }
                    }
                    bw.write_fmt(format_args!("{}\n", dp[2][m as usize]))
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

fn dfn(
    head: &[i32],
    next: &[i32],
    to: &[usize],
    vals: &[i32],
    dfn_vals: &mut [i32],
    size: &mut [usize],
    dfn_cnt: &mut usize,
    node: usize,
) -> usize {
    let cur_dfn_cnt = *dfn_cnt;
    *dfn_cnt += 1;
    dfn_vals[cur_dfn_cnt] = vals[node];
    size[cur_dfn_cnt] = 1;
    let mut nx = head[node];
    while nx > 0 {
        let sub = to[nx as usize];
        size[cur_dfn_cnt] += dfn(head, next, to, vals, dfn_vals, size, dfn_cnt, sub);
        nx = next[nx as usize];
    }
    size[cur_dfn_cnt]
}
