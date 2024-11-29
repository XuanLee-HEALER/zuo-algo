use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    println!("测试开始");
    for xlen in 0..1000 {
        let mut arr = random_array(xlen, 100);
        arr.sort();
        let r = rng.gen_bool(0.2);
        if r {
            // assert_eq!(search(&arr, 101), binary_search(&arr, 101));
            // assert_eq!(search_left(&arr, 101), binary_search_left(&arr, 101));
            assert_eq!(search_right(&arr, -1), binary_search_right(&arr, -1));
        } else {
            let in_range = rng.gen_range(0..=100);
            // assert_eq!(search(&arr, in_range), binary_search(&arr, in_range));
            // assert_eq!(
            //     search_left(&arr, in_range),
            //     binary_search_left(&arr, in_range)
            // );
            assert_eq!(
                search_right(&arr, in_range),
                binary_search_right(&arr, in_range)
            );
        }
    }
    println!("测试结束");
}

fn search(arr: &[i32], n: i32) -> bool {
    for (i, e) in arr.iter().enumerate() {
        if *e == n {
            return true;
        }
    }
    false
}

// 数列中大于n的最左位置
fn search_left(arr: &[i32], n: i32) -> Option<usize> {
    for (i, e) in arr.iter().enumerate() {
        if *e >= n {
            return Some(i);
        }
    }
    None
}

// 数列中小于n的最右位置
fn search_right(arr: &[i32], n: i32) -> Option<usize> {
    let mut res = Option::<usize>::None;
    for (i, e) in arr.iter().enumerate() {
        if *e <= n {
            res = Some(i);
        } else {
            break;
        }
    }
    res
}

fn binary_search(arr: &[i32], n: i32) -> bool {
    let len = arr.len();
    if len == 0 {
        return false;
    }

    let mut beg = 0;
    let mut end = len - 1;
    while beg <= end {
        let mid = beg + ((end - beg) >> 1);
        match arr[mid].cmp(&n) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => {
                beg = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                if mid > 0 {
                    end = mid - 1;
                } else {
                    break;
                }
            }
        }
    }

    false
}

fn binary_search_left(arr: &[i32], n: i32) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    let mut res = Option::<usize>::None;
    let mut beg = 0;
    let mut end = arr.len() - 1;
    while beg <= end {
        let mid = beg + ((end - beg) >> 1);
        match arr.get(mid).unwrap().cmp(&n) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                res = Some(mid);
                if mid > 0 {
                    end = mid - 1
                } else {
                    break;
                }
            }
            std::cmp::Ordering::Less => {
                beg = mid + 1;
            }
        }
    }

    res
}

fn binary_search_right(arr: &[i32], n: i32) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    let mut res = Option::<usize>::None;
    let mut beg = 0;
    let mut end = arr.len() - 1;
    while beg <= end {
        let mid = beg + ((end - beg) >> 1);
        match arr.get(mid).unwrap().cmp(&n) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {
                res = Some(mid);
                beg = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                if mid > 0 {
                    end = mid - 1;
                } else {
                    break;
                }
            }
        }
    }

    res
}

pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 {
        return 0;
    }

    if len > 1 {
        let beg = 0;
        let end = len - 1;
        if nums.get(beg).unwrap() > nums.get(beg + 1).unwrap() {
            return beg as i32;
        }
        if nums.get(end).unwrap() > nums.get(end - 1).unwrap() {
            return end as i32;
        }

        let mut beg = beg + 1;
        let mut end = end - 1;
        while beg <= end {
            let mid = beg + ((end - beg) >> 1);
            let mid_val = nums.get(mid).unwrap();
            let lcond = if mid > 0 {
                let mid_lval = nums.get(mid - 1).unwrap();
                mid_val > mid_lval
            } else {
                true
            };
            let mid_rval = nums.get(mid + 1).unwrap();
            let rcond = mid_val > mid_rval;
            if lcond && rcond {
                return mid as i32;
            } else if lcond {
                beg = mid + 1;
            } else if rcond {
                if mid > 0 {
                    end = mid - 1;
                } else {
                    break;
                }
            } else {
                beg = mid + 1;
            }
        }
    }

    -1
}

fn peak_problem(arr: &[i32]) -> Option<usize> {
    let len = arr.len();
    match len {
        0 => None,
        1 => Some(0),
        e => {
            let beg = 0;
            let end = e - 1;
            if arr.get(beg).unwrap() > arr.get(beg + 1).unwrap() {
                return Some(beg);
            }
            if arr.get(end).unwrap() > arr.get(end - 1).unwrap() {
                return Some(end);
            }

            let mut beg = beg + 1;
            let mut end = end - 1;
            while beg <= end {
                let mid = (beg >> 1) + (end >> 1) + (beg & end & 1);
                // let mid = beg + ((end - beg) >> 1);
                let mid_val = arr.get(mid).unwrap();
                let lcond = if mid > 0 {
                    let mid_lval = arr.get(mid - 1).unwrap();
                    mid_val > mid_lval
                } else {
                    true
                };
                let mid_rval = arr.get(mid + 1).unwrap();
                let rcond = mid_val > mid_rval;
                if lcond && rcond {
                    return Some(mid);
                } else if lcond {
                    beg = mid + 1;
                } else if rcond {
                    if mid > 0 {
                        end = mid - 1;
                    } else {
                        break;
                    }
                } else {
                    beg = mid + 1;
                }
            }
            None
        }
    }
}

// random_array n是数组长度，e是每个元素的最大值，生成长度为n的每个元素是0～e的数组
fn random_array(n: usize, e: i32) -> Vec<i32> {
    let mut res = Vec::<i32>::with_capacity(n);
    let mut rng = thread_rng();
    for _ in 0..n {
        res.push(rng.gen_range(0..=e));
    }
    res
}
