use std::time::Instant;

use rand::seq::SliceRandom;

fn main() {
    let mut nums = (1..=20_000_000).collect::<Vec<i32>>();
    nums.shuffle(&mut rand::thread_rng());
    println!("start to sort");
    let mut timer = Instant::now();
    heap_sort_1(&mut nums);
    println!("from top to down -> {}s", timer.elapsed().as_secs());
    timer = Instant::now();
    heap_sort_2(&mut nums.clone());
    println!("from down to top -> {}s", timer.elapsed().as_secs());
}

/// 从顶到底建堆
fn heap_sort_1(nums: &mut [i32]) {
    if nums.len() > 1 {
        for i in 0..nums.len() {
            heap_insert(nums, i)
        }

        let mut size = nums.len();
        while size > 0 {
            nums.swap(0, size - 1);
            heapify(nums, 0, size - 1);
            size -= 1;
        }
    }
}

/// 从低到顶建堆
fn heap_sort_2(nums: &mut [i32]) {
    if nums.len() > 1 {
        for i in (0..nums.len() - 1).rev() {
            heapify(nums, i, nums.len())
        }

        let mut size = nums.len();
        while size > 0 {
            nums.swap(0, size - 1);
            heapify(nums, 0, size - 1);
            size -= 1;
        }
    }
}

/// 保证数组的前`idx+1`和数是大根堆
fn heap_insert(arr: &mut [i32], mut idx: usize) {
    while idx > 0 {
        let p_idx = (idx - 1) / 2;
        if arr[p_idx] < arr[idx] {
            arr.swap(p_idx, idx);
            idx = p_idx;
        } else {
            return;
        }
    }
}

/// 在`size`的大小范围内保持大根堆
fn heapify(arr: &mut [i32], mut idx: usize, size: usize) {
    while idx < size {
        let left_idx = idx * 2 + 1;
        let right_idx = left_idx + 1;
        if left_idx < size && right_idx < size {
            if arr[left_idx] < arr[right_idx] {
                if arr[right_idx] > arr[idx] {
                    arr.swap(right_idx, idx);
                    idx = right_idx;
                } else {
                    return;
                }
            } else if arr[left_idx] > arr[idx] {
                arr.swap(left_idx, idx);
                idx = left_idx;
            } else {
                return;
            }
        } else if left_idx < size {
            if arr[left_idx] > arr[idx] {
                arr.swap(left_idx, idx);
                idx = left_idx;
            } else {
                return;
            }
        } else {
            return;
        }
    }
}
