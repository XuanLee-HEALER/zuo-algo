use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let mut vec1 = (-15..15).collect::<Vec<i32>>();
    vec1.shuffle(&mut rng);
    println!("ori: {vec1:?}");
    let res = sort_array(vec1);
    println!("res: {res:?}");
}

pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let min_val = nums.iter().min().unwrap();
    let mut nums: Vec<i32> = nums.iter().map(|v| v - min_val).collect();
    let max_val = nums.iter().max().unwrap();
    let max_bits = bits(*max_val);
    radix_sort(&mut nums, max_bits, 10);
    nums.iter().map(|v| v + min_val).collect()
}

fn bits(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        num /= 10;
        res += 1;
    }
    res
}

fn radix_sort(arr: &mut [i32], n: i32, base: i32) {
    let mut bit_arr = vec![0; base as usize];
    let mut aid = vec![0; arr.len()];
    let mut offset = 1;
    for _ in 0..n {
        for num in arr.iter() {
            let d = (*num / offset) % base;
            bit_arr[d as usize] += 1;
        }

        for i in 1..bit_arr.len() {
            bit_arr[i] += bit_arr[i - 1];
        }

        for i in (0..=arr.len() - 1).rev() {
            let d = (arr[i] / offset) % base;
            aid[(bit_arr[d as usize] - 1) as usize] = arr[i];
            bit_arr[d as usize] -= 1;
        }

        offset *= 10;

        arr.copy_from_slice(&aid);
        bit_arr.fill(0);
    }
}
