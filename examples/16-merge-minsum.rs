use std::io::{self, *};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut problem_arr = vec![0; 100_001];
    let mut aid_arr = vec![0; 100_001];
    let mut line = String::new();
    let mut count = 0;
    let mut arr_len: usize = 0;
    while let Ok(sz) = br.read_line(&mut line) {
        if sz > 0 {
            let trim_line = line.trim_end();

            if count % 2 == 0 {
                // length of array
                arr_len = trim_line.parse().unwrap();
            } else {
                // arr
                let splited: Vec<&str> = trim_line.split(" ").collect();
                for (i, v) in splited.iter().enumerate() {
                    problem_arr[i] = v.parse().unwrap();
                }

                let ans = minsum(&mut problem_arr, &mut aid_arr, 0, arr_len - 1);

                bw.write_fmt(format_args!("{}\n", ans)).unwrap()
                // bw.write_fmt(format_args!(
                //     "{}\n",
                //     problem_arr
                //         .iter()
                //         .take(arr_len)
                //         .map(|x| x.to_string())
                //         .collect::<Vec<String>>()
                //         .join(" ")
                // ))
                // .unwrap()
            }
            line.clear();
            count += 1;
        } else {
            break;
        }
    }
    bw.flush().unwrap();
}

fn minsum(nums: &mut [i32], aid: &mut [i32], l: usize, r: usize) -> i64 {
    if l == r {
        0
    } else {
        let mid = l + ((r - l) >> 1);
        minsum(nums, aid, l, mid)
            + minsum(nums, aid, mid + 1, r)
            + merge_with_count(nums, aid, l, r, mid + 1)
    }
}

fn merge_with_count(nums: &mut [i32], aid: &mut [i32], l: usize, r: usize, mid: usize) -> i64 {
    let mut res = 0;
    let mut tc = 0;
    let mut tl = l;
    let mut tr = mid;
    while tr <= r {
        while tl < mid {
            if nums[tl] <= nums[tr] {
                tc += nums[tl] as i64;
                tl += 1;
            } else {
                break;
            }
        }
        res += tc;
        tr += 1;
    }

    let mut count = 0;
    let mut tl = l;
    let mut tr = mid;
    while tl < mid && tr <= r {
        if nums[tl] <= nums[tr] {
            aid[count] = nums[tl];
            tl += 1;
        } else {
            aid[count] = nums[tr];
            tr += 1;
        }
        count += 1;
    }

    while tl < mid {
        aid[count] = nums[tl];
        tl += 1;
        count += 1;
    }

    while tr <= r {
        aid[count] = nums[tr];
        tr += 1;
        count += 1;
    }

    nums[l..=r].copy_from_slice(&aid[..count]);

    res
}
