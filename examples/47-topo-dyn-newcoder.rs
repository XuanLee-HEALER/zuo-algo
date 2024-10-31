use std::io::{self, BufRead, BufReader, BufWriter, Write};

// 运行时间 78ms 占用内存 17656KB
fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut n = 0;
    let mut m = 0;
    let mut head = false;
    let mut graph: Vec<Vec<usize>> = Vec::new();
    let mut indegree: Vec<usize> = Vec::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                graph = vec![vec![0; 0]; n + 1];
                indegree = vec![0; n + 1];
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2 = segs.next().unwrap().parse().unwrap();
                graph[p1].push(p2);
                indegree[p2] += 1;
                m -= 1;

                if m == 0 {
                    let mut ans = vec![0; n];
                    if topo(n, &graph, &mut indegree, &mut ans) {
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

fn topo(n: usize, graph: &[Vec<usize>], indegree: &mut [usize], ans: &mut [i32]) -> bool {
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
        for &other in &graph[q[l]] {
            if indegree[other] == 1 {
                q[r] = other;
                r += 1;
            }
            indegree[other] -= 1;
        }
        l += 1;
        cnt += 1;
    }

    n == cnt
}
