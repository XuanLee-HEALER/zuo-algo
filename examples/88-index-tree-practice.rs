fn main() {
    println!("res {}", Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]));
    println!(
        "res {}",
        Solution::min_moves_to_make_palindrome("aabb".into())
    );
}

struct Solution;

impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let n = s.len();
        struct LastLoc {
            ends: [usize; 26],
            loc: Vec<usize>,
        }
        impl LastLoc {
            fn idx(u: u8) -> usize {
                (u - b'a') as usize
            }

            fn from(s: &str) -> Self {
                let mut ends = [0; 26];
                let mut loc = vec![0; s.len() + 1];
                for (i, &b) in s.as_bytes().iter().enumerate() {
                    let idx = Self::idx(b);
                    loc[i + 1] = ends[idx];
                    ends[idx] = i + 1;
                }
                Self { ends, loc }
            }

            fn last_loc(&mut self, u: u8) -> usize {
                let idx = Self::idx(u);
                let r = self.ends[idx];
                self.ends[idx] = self.loc[r];
                r
            }
        }
        let mut last_loc = LastLoc::from(&s);
        fn bk(i: i32) -> i32 {
            i & -i
        }
        fn add(arr: &mut [i32], n: usize, i: usize, v: i32) {
            let mut i = i as i32;
            while i <= n as i32 {
                arr[i as usize] += v;
                i += bk(i);
            }
        }
        fn sum(arr: &[i32], i: usize) -> i32 {
            let mut res = 0;
            let mut i = i as i32;
            while i > 0 {
                res += arr[i as usize];
                i -= bk(i);
            }
            res
        }
        let mut itree = vec![0; n + 1];
        // ⚠️初始化树状数组
        for i in 1..=n {
            add(&mut itree, n, i, 1);
        }
        let mut final_arr = vec![0; n + 1];
        for (i, &b) in s.as_bytes().iter().enumerate() {
            // 最终的位置还没定的字符
            if final_arr[i + 1] == 0 {
                // 原来的位置
                let l = i + 1;
                // 右边离最远的字符的位置
                let r = last_loc.last_loc(b);
                if l < r {
                    // 原来位置应该放的位置
                    let k = sum(&itree, l);
                    final_arr[l] = k;
                    // 最右边那个字符应该放的位置
                    final_arr[r] = n as i32 - k + 1;
                } else if l == r {
                    final_arr[l] = ((n + 1) >> 1) as i32;
                }
                // 被移除的字符的位置减1
                add(&mut itree, n, r, -1);
            }
        }

        fn inverse_pairs(n: usize, arr: Vec<i32>) -> usize {
            let mut aid = vec![0; n];
            let mut arr = arr;
            f(&mut arr, 1, n, &mut aid)
        }

        fn f(arr: &mut [i32], l: usize, r: usize, aid: &mut [i32]) -> usize {
            if l == r {
                0
            } else {
                let m = l + ((r - l) >> 1);
                f(arr, l, m, aid) + f(arr, m + 1, r, aid) + merge(arr, l, m, r, aid)
            }
        }

        fn merge(arr: &mut [i32], l: usize, m: usize, r: usize, aid: &mut [i32]) -> usize {
            let mut res = 0;
            let mut r1 = r;
            for l1 in (l..=m).rev() {
                while r1 > m && arr[l1] <= arr[r1] {
                    r1 -= 1;
                }
                if r1 <= m {
                    break;
                } else {
                    res += r1 - m
                }
            }

            let (mut l1, mut r1) = (l, m + 1);
            let mut ct = 0;
            while l1 <= m && r1 <= r {
                if arr[l1] < arr[r1] {
                    aid[ct] = arr[l1];
                    l1 += 1;
                } else {
                    aid[ct] = arr[r1];
                    r1 += 1;
                }
                ct += 1;
            }
            while l1 <= m {
                aid[ct] = arr[l1];
                l1 += 1;
                ct += 1;
            }
            while r1 <= r {
                aid[ct] = arr[r1];
                r1 += 1;
                ct += 1;
            }
            arr[l..=r].copy_from_slice(&mut aid[..ct]);
            res
        }
        inverse_pairs(n, final_arr) as i32
    }

    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let d_arr = Self::discrate_array(&nums);
        let n = nums.len();
        let mut itree = vec![(0, 0); n + 1];
        nums.iter_mut()
            .for_each(|v| *v = (d_arr[1..].binary_search(v).unwrap() + 1) as i32);
        for &v in &nums {
            let (ml, mc) = Self::sum(&itree, v - 1);
            Self::add(&mut itree, n as i32, v, ml + 1, mc.max(1));
        }
        Self::sum(&itree, n as i32).1 as i32
    }

    fn add(arr: &mut [(usize, usize)], n: i32, mut i: i32, ml: usize, mc: usize) {
        while i <= n {
            let (cl, _) = arr[i as usize];
            if ml > cl {
                arr[i as usize] = (ml, mc)
            } else if ml == cl {
                arr[i as usize].1 += mc
            }
            i += Self::bk(i)
        }
    }

    // 小于等于某个数的最长递增子序列长度，和这些最长子序列的数量
    fn sum(arr: &[(usize, usize)], mut i: i32) -> (usize, usize) {
        let (mut ml, mut mc) = (0, 0);
        while i > 0 {
            let (l, c) = arr[i as usize];
            if l > ml {
                ml = l;
                mc = c;
            } else if l == ml {
                mc += c
            }
            i -= Self::bk(i);
        }
        (ml, mc)
    }

    fn bk(i: i32) -> i32 {
        i & -i
    }

    fn discrate_array(ori: &[i32]) -> Vec<i32> {
        let n = ori.len();
        let mut res = vec![0; n + 1];
        res[1..].copy_from_slice(&ori);
        res[1..].sort_unstable();
        let mut cur = 2;
        let mut nxt = cur;
        while nxt <= n {
            if res[nxt] != res[nxt - 1] {
                res[cur] = res[nxt];
                cur += 1;
            }
            nxt += 1;
        }
        res[..cur].into()
    }
}
