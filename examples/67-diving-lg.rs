use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut m = 0;
    let mut v = 0;
    let mut n = 0;
    let mut tools = vec![];
    let mut counter = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                m = segs.next().unwrap().parse().unwrap();
                v = segs.next().unwrap().parse().unwrap();
                n = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let a = segs.next().unwrap().parse().unwrap();
                let b = segs.next().unwrap().parse().unwrap();
                let c = segs.next().unwrap().parse().unwrap();
                tools.push((a, b, c));
                counter += 1;

                if counter == n {
                    let mut dp = vec![vec![0; v + 1]; m + 1];
                    let mut path = vec![vec!["".to_string(); v + 1]; m + 1];
                    let t_res = compute(&tools, m, v, &mut dp, &mut path);

                    bw.write_fmt(format_args!("{}\n", t_res)).unwrap();
                    bw.write_fmt(format_args!("{}\n", path[m][v].trim()))
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
    tools: &[(i32, i32, i32)],
    m: usize,
    v: usize,
    dp: &mut [Vec<i32>],
    path: &mut [Vec<String>],
) -> i32 {
    for (i, &(a, b, c)) in tools.iter().enumerate() {
        for j in (0..=m).rev() {
            for k in (0..=v).rev() {
                if j as i32 >= a && k as i32 >= b {
                    if c + dp[j - a as usize][k - b as usize] > dp[j][k] {
                        dp[j][k] = c + dp[j - a as usize][k - b as usize];
                        path[j][k] = if path[j - a as usize][k - b as usize] == "".to_string() {
                            (i + 1).to_string()
                        } else {
                            format!("{} {}", path[j - a as usize][k - b as usize], i + 1)
                        };
                    } else if c + dp[j - a as usize][k - b as usize] == dp[j][k] {
                        let new_path = if path[j - a as usize][k - b as usize] == "".to_string() {
                            (i + 1).to_string()
                        } else {
                            format!("{} {}", path[j - a as usize][k - b as usize], i + 1)
                        };
                        if new_path < path[j][k] {
                            path[j][k] = new_path
                        }
                    }
                }
            }
        }
    }
    dp[m][v]
}
