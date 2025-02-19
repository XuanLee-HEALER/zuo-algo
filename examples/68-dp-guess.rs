fn main() {
    // println!("res: {:?}", solve(30, 10));
    // println!("matcher: {:?}", solve1(30, 10));

    // println!("res: {:?}", solve(10, 5));
    // println!("matcher: {:?}", solve1(10, 5));
    // assert_eq!(
    //     1,
    //     Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4])
    // );
    // assert_eq!(
    //     2,
    //     Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1])
    // );
    assert_eq!(
        -1,
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3])
    )
}

struct Solution;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        arr2.sort();
        arr2.dedup();
        let n = arr1.len();
        let mut dp = vec![-1; n];
        let res = Self::f(&arr1, &arr2, n, arr2.len(), 0, &mut dp);
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }

    fn f(arr1: &[i32], arr2: &[i32], n: usize, m: usize, i: usize, dp: &mut [i32]) -> i32 {
        if i == n {
            // i位置以及之后已经没有数字了，直接满足条件，所以需要操作是0
            0
        } else if dp[i] != -1 {
            dp[i]
        } else {
            let mut res = i32::MAX;
            let mut pre = if i == 0 { i32::MIN } else { arr1[i - 1] };
            let mut find = Self::bs_max_left(arr2, pre);
            for j in i..=n {
                if j == n {
                    res = res.min((j - i) as i32);
                } else {
                    if arr1[j] > pre {
                        let next = Self::f(arr1, arr2, n, m, j + 1, dp);
                        if next != i32::MAX {
                            res = res.min(next + (j - i) as i32)
                        }
                    }

                    if let Some(x_find) = find {
                        if x_find < m {
                            pre = arr2[x_find];
                            find = Some(x_find + 1)
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            dp[i] = res;
            res
        }
    }

    fn bs_max_left(arr: &[i32], num: i32) -> Option<usize> {
        let mut res = None;
        if arr.len() == 0 {
            res
        } else {
            let mut beg = 0;
            let mut end = arr.len() - 1;
            while beg <= end {
                let mid = beg + ((end - beg) >> 1);
                if arr[mid] > num {
                    res = Some(mid);
                    if mid > 0 {
                        end = mid - 1;
                    } else {
                        break;
                    }
                } else {
                    beg = mid + 1;
                }
            }
            res
        }
    }
}

fn solve1(n: i32, k: i32) -> Vec<i32> {
    let res = vec![];
    let all_sum = n * (n + 1) / 2;
    let mut dp = vec![vec![vec![-1; all_sum as usize / 2 + 2]; k as usize + 1]; n as usize + 1];
    let mut t_arr = vec![];
    let mut tt_arr = vec![];
    let mut res1 = sub(n, 1, k, all_sum / 2, &mut dp, &mut t_arr, &mut tt_arr);
    for arr in &tt_arr {
        println!("tmp_res: {:?}", arr)
    }
    // if !res1 && all_sum % 2 == 1 {
    //     res1 = sub(n, 1, k, all_sum / 2 + 1)
    // }
    res
}

fn sub(
    n: i32,
    i: i32,
    k: i32,
    sum: i32,
    dp: &mut Vec<Vec<Vec<i32>>>,
    t_arr: &mut Vec<i32>,
    tt_arr: &mut Vec<Vec<i32>>,
) -> bool {
    if i > n {
        if k == 0 && sum == 0 {
            tt_arr.push(t_arr.clone());
            true
        } else {
            false
        }
    } else if k == 0 {
        if sum == 0 {
            tt_arr.push(t_arr.clone());
            true
        } else {
            false
        }
    // } else if dp[i as usize][k as usize][sum as usize] != -1 {
    //     if dp[i as usize][k as usize][sum as usize] == 0 {
    //         false
    //     } else {
    //         true
    //     }
    } else if sum < 0 {
        false
    } else {
        let p1 = sub(n, i + 1, k, sum, dp, t_arr, tt_arr);
        t_arr.push(i);
        let p2 = sub(n, i + 1, k - 1, sum - i, dp, t_arr, tt_arr);
        t_arr.pop();
        dp[i as usize][k as usize][sum as usize] = if p1 || p2 { 1 } else { 0 };
        p1 || p2
    }
}

fn solve(n: i32, k: i32) -> Vec<i32> {
    let all_sum = n * (n + 1) / 2;
    let mut res1 = generate(n, k, all_sum / 2);
    if res1.is_empty() && all_sum % 2 == 1 {
        res1 = generate(n, k, all_sum / 2 + 1)
    }
    res1
}

fn generate(n: i32, k: i32, sum: i32) -> Vec<i32> {
    let mut res = vec![];
    let min_sum = k * (k + 1) / 2;
    let add_range = n - k;
    let max_sum = min_sum + (k * add_range);
    if sum >= min_sum && sum <= max_sum {
        let t_sum = sum - min_sum;
        let num = t_sum / add_range;
        let rem = t_sum - (num * add_range);
        let mid = (k - num) + rem;
        let left = k - num - 1;
        for i in 1..=left {
            res.push(i);
        }
        res.push(mid);
        for i in (0..num).rev() {
            res.push(n - i);
        }
    }
    res
}
