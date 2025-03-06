use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

fn main() {
    let res = Solution::exponentiation_by_squaring(2, 3, Solution::MOD);
    // println!("res: {}", res);
    println!("res: {}", Solution::dumb_n_divided_by_k(17, 12));
}

struct Solution;

impl Solution {
    const MOD: i32 = 1_000_000_007;

    fn exponentiation_by_squaring(n: i32, mut exp: i32, rem: i32) -> i32 {
        let mut res = 1;
        let mut x = n;
        while exp > 0 {
            if exp & 1 > 0 {
                res = res * x % rem
            }
            x = x * x % rem;
            exp >>= 1
        }
        res
    }

    pub fn integer_break(n: i32) -> i32 {
        match n {
            n if n <= 1 => panic!("error"),
            2 => 1,
            3 => 2,
            4 => 4,
            def => {
                let r = def % 3;
                let mut exp = 0;
                let mut tail = 0;
                if r == 0 {
                    exp = def / 3;
                    tail = 1;
                } else if r == 1 {
                    exp = (def - 4) / 3;
                    tail = 4;
                } else if r == 2 {
                    exp = def / 3;
                    tail = 2;
                }
                Self::exponentiation_by_squaring(3, exp, Self::MOD) * tail % Self::MOD
            }
        }
    }

    fn n_divided_by_k(n: i32, k: i32) -> i32 {
        let base = n / k;
        let first = n % k;
        let second = k - first;
        let first_res = Self::exponentiation_by_squaring(base + 1, first, Self::MOD);
        let second_res = Self::exponentiation_by_squaring(base, second, Self::MOD);
        (first_res as i64 * second_res as i64 % Self::MOD as i64) as i32
    }

    fn dumb_n_divided_by_k(n: i32, k: i32) -> i32 {
        if k == 0 {
            1
        } else {
            let mut res = 0;
            for x in 1..=(n - k + 1) {
                res = res.max(x * Self::dumb_n_divided_by_k(n - x, k - 1))
            }
            res
        }
    }

    fn max_meeting(meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        meetings.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut res = 0;
        let mut pre_max = i32::MIN;
        for meeting in meetings {
            if meeting[0] >= pre_max {
                res += 1;
                pre_max = meeting[1]
            }
        }
        res
    }

    fn dumb_max_meeting(meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        let mut res = 0;
        Self::permutation_max_meeting(&mut meetings, 0, &mut res);
        res
    }

    fn permutation_max_meeting(meetings: &mut Vec<Vec<i32>>, i: usize, max: &mut i32) {
        if i == meetings.len() {
            let mut res = 0;
            let mut pre_max = i32::MIN;
            for meeting in meetings.iter() {
                if meeting[0] >= pre_max {
                    res += 1;
                    pre_max = meeting[1];
                }
            }
            if res > *max {
                *max = res
            }
        } else {
            for j in i..meetings.len() {
                meetings.swap(i, j);
                Self::permutation_max_meeting(meetings, i + 1, max);
                meetings.swap(i, j);
            }
        }
    }

    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut end = 0;
        for event in events.iter() {
            if event[1] > end {
                end = event[1]
            }
        }
        let mut events = events;
        events.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let beg = events[0][0];
        let mut min_heap = BinaryHeap::new();
        let mut res = 0;
        let mut j = 0;
        for i in beg..=end {
            while j < events.len() && events[j][0] == i {
                min_heap.push(Reverse(events[j][1]));
                j += 1;
            }

            while !min_heap.is_empty() && min_heap.peek().unwrap().0 < i {
                min_heap.pop();
            }

            if !min_heap.is_empty() {
                res += 1;
                min_heap.pop();
            }
        }
        res
    }

    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut capital = capital
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        capital.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let mut max_heap = BinaryHeap::new();
        let mut k = k;
        let mut w = w;
        let mut ti = 0;
        while k > 0 {
            while ti < capital.len() {
                if capital[ti].1 <= w {
                    max_heap.push(profits[capital[ti].0]);
                    ti += 1;
                } else {
                    break;
                }
            }

            if !max_heap.is_empty() {
                w += max_heap.pop().unwrap();
                k -= 1;
            } else {
                break;
            }
        }

        w
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    fn number_of_array(arr: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_cnt = 0;
        let mut ct = HashMap::new();
        let g = 0;
        for &i in &arr {
            if i != 0 {
                if i > max {
                    max = i;
                }
            }
            ct.entry(i).and_modify(|v| *v += 1).or_insert(1);
            let cur_cnt = *ct.get(&i).unwrap();
            if cur_cnt > max_cnt {
                max_cnt = cur_cnt
            }
        }
        if max == 0 {
            arr.len() as i32
        } else {
            let mut res = arr.len();
            let g = arr
                .into_iter()
                .reduce(|a, b| {
                    if a > b {
                        Self::gcd(a, b)
                    } else {
                        Self::gcd(b, a)
                    }
                })
                .unwrap();
            let mut ini = g;
            while ini <= max {
                if !ct.contains_key(&ini) {
                    res += 1
                }
                ini += g;
            }
            if ct.get(&0).is_none() {
                if max_cnt > 1 {
                    res += 1;
                }
            }
            res as i32
        }
    }

    fn dumb_number_of_array(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        let mut hs = HashSet::new();
        for &i in &arr {
            hs.insert(i);
        }
        while !Self::dnoa(&mut arr, &mut hs) {}
        arr.len() as i32
    }

    fn dnoa(arr: &mut Vec<i32>, check: &mut HashSet<i32>) -> bool {
        let ori = arr.len();
        for i in 0..ori {
            for j in i + 1..ori {
                let sub = (arr[i] - arr[j]).abs();
                if !check.contains(&sub) {
                    arr.push(sub);
                    check.insert(sub);
                }
            }
        }
        ori == arr.len()
    }
}

#[cfg(test)]
mod test_solution {
    use rand::{thread_rng, Rng};

    use crate::Solution;

    #[test]
    fn test_n_divided_by_k() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(2..=30);
            let k = rng.gen_range(1..=n);
            let bright = Solution::n_divided_by_k(n, k);
            let dumb = Solution::dumb_n_divided_by_k(n, k);
            assert_eq!(
                bright, dumb,
                "wrong anwser: bright{}, dumb{}\nn is {}, k is {}",
                bright, dumb, n, k
            )
        }
    }

    #[test]
    fn test_max_meeting() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(2..=10);
            let mut meetings = Vec::new();
            for _ in 0..n {
                let start = rng.gen_range(1..=300);
                let end = rng.gen_range(start + 1..10000);
                meetings.push(vec![start, end]);
            }
            let bright = Solution::max_meeting(meetings.clone());
            let dumb = Solution::dumb_max_meeting(meetings);
            assert_eq!(bright, dumb, "wrong anwser: bright{}, dumb{}", bright, dumb)
        }
    }

    #[test]
    fn test_number_of_array() {
        let mut rng = thread_rng();
        for _ in 0..20_000 {
            let n = rng.gen_range(2..=100);
            let mut v = vec![];
            for _ in 0..n {
                let m = rng.gen_range(0..=100);
                v.push(m);
            }
            let bright = Solution::number_of_array(v.clone());
            let dumb = Solution::dumb_number_of_array(v.clone());
            assert_eq!(
                bright, dumb,
                "wrong anwser: bright {}, dumb {}\narray is {:?}",
                bright, dumb, v
            )
        }
    }
}
