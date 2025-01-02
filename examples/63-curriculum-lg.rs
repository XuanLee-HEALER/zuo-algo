use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut m = 0;
    let mut cur = 1;
    let mut graph = vec![];
    let mut vals = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                graph = vec![vec![]; n + 1];
                vals = vec![0; n + 1];
                m += 1;
                head = true;
            } else {
                let par: usize = segs.next().unwrap().parse().unwrap();
                let score = segs.next().unwrap().parse().unwrap();
                graph[par].push(cur);
                vals[cur] = score;
                cur += 1;
                if cur > n {
                    let mut dp = vec![vec![vec![-1; m + 1]; n]; n + 1];
                    bw.write_fmt(format_args!(
                        "{}\n",
                        compute(&graph, &vals, 0, graph[0].len(), m, &mut dp)
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

fn compute(
    graph: &[Vec<usize>],
    vals: &[i32],
    i: usize,
    j: usize,
    k: usize,
    dp: &mut [Vec<Vec<i32>>],
) -> i32 {
    if k == 0 {
        0
    } else if j == 0 {
        vals[i]
    } else if dp[i][j][k] != -1 {
        dp[i][j][k]
    } else {
        // 不选第j个子树
        let mut ans = compute(graph, vals, i, j - 1, k, dp);
        let j_sub = graph[i][j - 1];
        for s in 1..k {
            ans = ans.max(
                compute(graph, vals, i, j - 1, k - s, dp)
                    + compute(graph, vals, j_sub, graph[j_sub].len(), s, dp),
            );
        }
        dp[i][j][k] = ans;
        dp[i][j][k]
    }
}
