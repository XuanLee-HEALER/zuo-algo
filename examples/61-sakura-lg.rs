use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MAX_NUMS: usize = 1_000;
fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut c_limit = 0;
    let group_unit = usize::next_power_of_two(MAX_NUMS);
    let mut costs = vec![];
    let mut vals = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            let mut segs = buf.trim().split(" ");
            if !head {
                let start = segs.next().unwrap();
                let end = segs.next().unwrap();
                n = segs.next().unwrap().parse().unwrap();
                c_limit = limit(start, end);
                costs = Vec::with_capacity(n * group_unit);
                vals = Vec::with_capacity(n * group_unit);
                head = true;
            } else {
                let cost: i32 = segs.next().unwrap().parse().unwrap();
                let val: i32 = segs.next().unwrap().parse().unwrap();
                let mut num: i32 = segs.next().unwrap().parse().unwrap();
                num = if num == 0 { 1000 } else { num };
                let mut k = 1;
                while k <= num {
                    costs.push(k * cost);
                    vals.push(k * val);
                    num -= k;
                    k <<= 1;
                }
                if num > 0 {
                    costs.push(num * cost);
                    vals.push(num * val);
                }
                n -= 1;

                if n == 0 {
                    bw.write_fmt(format_args!("{}\n", compute(&costs, &vals, c_limit)))
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

fn limit(start: &str, end: &str) -> usize {
    let start = start
        .split(":")
        .enumerate()
        .map(|(i, s)| if i == 0 { 60 } else { 1 } * (s.parse::<usize>().unwrap()))
        .reduce(|a, b| a + b)
        .unwrap();
    let end = end
        .split(":")
        .enumerate()
        .map(|(i, s)| if i == 0 { 60 } else { 1 } * (s.parse::<usize>().unwrap()))
        .reduce(|a, b| a + b)
        .unwrap();
    end - start
}

fn compute(costs: &[i32], vals: &[i32], limit: usize) -> i32 {
    let mut dp = vec![0; limit + 1];
    for (i, &cost) in costs.iter().enumerate() {
        for j in (0..=limit).rev() {
            dp[j] = dp[j].max(if j as i32 >= cost {
                dp[(j as i32 - cost) as usize] + vals[i]
            } else {
                0
            })
        }
    }
    dp[limit]
}

#[cfg(test)]
mod test_sakura {
    #[test]
    fn test_limit() {
        assert_eq!(super::limit("6:50", "7:00"), 10);
        assert_eq!(super::limit("6:20", "7:30"), 70);
    }
}
