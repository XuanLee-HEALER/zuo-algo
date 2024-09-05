use std::{
    cmp,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

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

                merge_sort(&mut problem_arr, &mut aid_arr, 0, arr_len - 1);

                bw.write_fmt(format_args!(
                    "{}\n",
                    problem_arr
                        .iter()
                        .take(arr_len)
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
                .unwrap()
            }
            line.clear();
            count += 1;
        } else {
            break;
        }
    }
    bw.flush().unwrap();
}

fn merge_sort(nums: &mut [i32], aid: &mut [i32], l: usize, r: usize) {
    // merge_sort_rec(nums, l, r, aid)
    merge_sort_no_rec(nums, aid, l, r)
}

fn merge_sort_no_rec(nums: &mut [i32], aid: &mut [i32], l: usize, r: usize) {
    let mut step = 1;
    let arr_len = r - l + 1;
    while step < arr_len {
        let mut l = 0;
        while l < arr_len {
            let il = l;
            let mid = il + step;
            if mid >= arr_len {
                break;
            }

            let ir = cmp::min(mid + step - 1, arr_len - 1);
            merge(nums, aid, il, ir, mid - 1);

            l = ir + 1;
        }

        step <<= 1;
    }
}

fn merge_sort_rec(nums: &mut [i32], l: usize, r: usize, aid: &mut [i32]) {
    if l != r {
        let mid = l + ((r - l) >> 1);
        merge_sort_rec(nums, l, mid, aid);
        merge_sort_rec(nums, mid + 1, r, aid);
        merge(nums, aid, l, r, mid);
    }
}

fn merge(nums: &mut [i32], aid: &mut [i32], l: usize, r: usize, mid: usize) {
    let mut il = l;
    let mut ir = mid + 1;

    let mut i = 0;
    while il <= mid && ir <= r {
        if nums[il] <= nums[ir] {
            aid[i] = nums[il];
            il += 1;
        } else {
            aid[i] = nums[ir];
            ir += 1;
        }
        i += 1;
    }

    while il <= mid {
        aid[i] = nums[il];
        il += 1;
        i += 1;
    }

    while ir <= r {
        aid[i] = nums[ir];
        ir += 1;
        i += 1;
    }

    nums[l..=r].copy_from_slice(&aid[..i])
}
