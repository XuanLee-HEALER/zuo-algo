use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());

    let mut buf = String::new();
    let mut problem_arr = Vec::with_capacity(100_001);
    let mut head = false;
    let k: i32 = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                // read head
                head = true;
                // let mut iter = buf.trim().split(" ");
                // iter.next();
                // k = iter.next().unwrap().parse().unwrap();
            } else {
                buf.trim()
                    .split(" ")
                    .for_each(|e| problem_arr.push(e.parse().unwrap()));
                // bw.write_fmt(format_args!("{}\n", sub_length(&problem_arr, k)))
                // .unwrap();
                bw.write_fmt(format_args!("{}\n", zero_one_length(&problem_arr)))
                    .unwrap();
                problem_arr.clear();
                head = false;
            }

            buf.clear();
        } else {
            break;
        }
    }

    bw.flush().unwrap();
}

fn zero_one_length(arr: &[i32]) -> i32 {
    let arr: Vec<i32> = arr
        .iter()
        .map(|i| match i.cmp(&0) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
        })
        .collect();
    let mut pre_loc: HashMap<i32, i32> = HashMap::new();
    pre_loc.insert(0, -1);

    let mut ans = 0;
    let mut sum = 0;
    for (idx, num) in arr.iter().enumerate() {
        sum += num;
        let aim = sum;

        if let Some(v) = pre_loc.get(&aim) {
            ans = ans.max(idx as i32 - *v)
        }

        pre_loc.entry(sum).or_insert(idx as i32);
    }

    ans
}

fn sub_length(arr: &[i32], k: i32) -> i32 {
    let mut pre_loc: HashMap<i32, i32> = HashMap::new();
    pre_loc.insert(0, -1);

    let mut ans = 0;
    let mut sum = 0;
    for (idx, num) in arr.iter().enumerate() {
        sum += num;
        let aim = sum - k;

        if let Some(v) = pre_loc.get(&aim) {
            ans = ans.max(idx as i32 - *v)
        }

        pre_loc.entry(sum).or_insert(idx as i32);
    }

    ans
}
