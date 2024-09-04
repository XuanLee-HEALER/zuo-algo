fn main() {
    let ans = Solution::reverse_pairs(vec![1, 3, 2, 3, 1]);
    println!("answer is {ans}");
}

struct Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let r = nums.len() - 1;
        Self::count(&mut nums, 0, r)
    }

    fn count(nums: &mut [i32], l: usize, r: usize) -> i32 {
        if l == r {
            0
        } else {
            let mid = l + ((r - l) >> 1);
            Self::count(nums, l, mid)
                + Self::count(nums, mid + 1, r)
                + Self::merge_with_count(nums, l, r, mid + 1)
        }
    }

    fn merge_with_count(nums: &mut [i32], l: usize, r: usize, mid: usize) -> i32 {
        let mut res = 0;
        let mut tc = 0;
        let mut tl = l;
        let mut tr = mid;
        while tl < mid {
            while tr <= r {
                if nums[tl] as i64 > (nums[tr] as i64) * 2 {
                    tc += 1;
                    tr += 1;
                } else {
                    break;
                }
            }
            res += tc;
            tl += 1;
        }

        let mut aid = vec![0; r - l + 1];
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
}
