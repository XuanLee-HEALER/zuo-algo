use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    buf.clear();
    for _ in 0..n {
        br.read_line(&mut buf).unwrap();
        buf.clear();
        br.read_line(&mut buf).unwrap();
        // let ori = buf
        //     .trim()
        //     .split(" ")
        //     .map(|v| v.parse::<i32>().unwrap())
        //     .reduce(|a, b| a ^ b)
        //     .unwrap();
        // buf.clear();
        let ori = buf
            .trim()
            .split(" ")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        // bw.write_fmt(format_args!("{}\n", if ori == 0 { "No" } else { "Yes" }))
        //     .unwrap();
        bw.write_fmt(format_args!("{}\n", sg_nim(&ori))).unwrap();
    }
    bw.flush().unwrap();
}

fn sg_nim(stones: &[i32]) -> &'static str {
    let &max = stones.iter().max().unwrap();
    let mut sg = vec![0; max as usize + 1];
    let mut appear = vec![false; max as usize + 1];
    for i in 1..=(max as usize) {
        appear.fill(false);
        for j in 1..=(max as usize) {
            if j <= i {
                appear[sg[i - j] as usize] = true
            } else {
                break;
            }
        }
        for (ti, &b) in appear.iter().enumerate() {
            if !b {
                sg[i] = ti as i32;
                break;
            }
        }
    }
    let mut xor = 0;
    for &stone in stones {
        xor ^= sg[stone as usize]
    }

    // 根据打表可以发现尼姆博弈的sg值 sg[i] = i
    println!("sg:\n{:?}", sg);

    if xor == 0 { "No" } else { "Yes" }
}
