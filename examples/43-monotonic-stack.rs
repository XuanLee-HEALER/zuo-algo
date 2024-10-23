use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut ori_arr = vec![0; 1_000_001];
    let mut stack = vec![0; 1_000_001];
    let mut ans = vec![(0, 0); 1_000_001];
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                head = true;
            } else {
                let mut counter = 0;
                buf.trim().split(" ").for_each(|s| {
                    ori_arr[counter] = s.parse().unwrap();
                    counter += 1;
                });
                calculate(&ori_arr[0..n], &mut stack, &mut ans);

                for &(e0, e1) in &ans[0..n] {
                    bw.write_fmt(format_args!("{} {}\n", e0, e1)).unwrap();
                }
                head = false;
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn calculate(arr: &[i32], stack: &mut [i32], ans: &mut [(i32, i32)]) {
    let mut top = 0;

    for (idx, &e) in arr.iter().enumerate() {
        if top == 0 || e > arr[stack[top - 1] as usize] {
            stack[top] = idx as i32;
            top += 1;
        } else {
            while top > 0 && e <= arr[stack[top - 1] as usize] {
                ans[stack[top - 1] as usize] =
                    (if top > 1 { stack[top - 2] } else { -1 }, idx as i32);
                top -= 1;
            }
            stack[top] = idx as i32;
            top += 1;
        }
    }

    while top > 0 {
        ans[stack[top - 1] as usize] = (if top > 1 { stack[top - 2] } else { -1 }, -1);
        top -= 1;
    }

    if arr.len() >= 2 {
        for i in (0..=arr.len() - 2).rev() {
            if ans[i].1 != -1 && arr[ans[i].1 as usize] == arr[i] {
                ans[i].1 = ans[ans[i].1 as usize].1
            }
        }
    }
}
