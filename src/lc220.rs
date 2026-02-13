use std::collections::HashMap;

struct Solution;

impl Solution {
    // 使用桶排序，数值空间排布到 value_diff+1 个桶中
    // 维护一个长度为 index_diff 的滑动窗口
    // 每个数都会进入桶 nums[i]/(value_diff+1)
    // 同一个桶内差值一定小于等于 value_diff，所以如果桶已有元素直接返回 true
    // 相邻桶内可能出现小于等于 value_diff 的情况，所以左右邻桶需要判断
    // 每个桶最多只能有1个元素，因为超出1个就已经返回 true 了（重要前提）
    // 如果窗口长度超出 index_diff，那么将最左边元素的桶删除
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let w = value_diff as i64 + 1; // 桶宽
        let k = index_diff as usize;
        let mut buckets: HashMap<i64, i64> = HashMap::new();

        // 求桶编号，负数需要偏移使得 [-w, -1] 映射到 -1 号桶
        let bucket_id = |v: i64| -> i64 {
            if v >= 0 { v / w } else { (v + 1) / w - 1 }
        };

        for i in 0..nums.len() {
            let v = nums[i] as i64;
            let id = bucket_id(v);
            // 同桶：差值一定 <= value_diff
            if buckets.contains_key(&id) {
                return true;
            }
            // 左邻桶
            if let Some(&bv) = buckets.get(&(id - 1)) {
                if (v - bv).abs() <= value_diff as i64 {
                    return true;
                }
            }
            // 右邻桶
            if let Some(&bv) = buckets.get(&(id + 1)) {
                if (v - bv).abs() <= value_diff as i64 {
                    return true;
                }
            }
            buckets.insert(id, v);
            // 窗口超出 index_diff，移除最左边元素
            if i >= k {
                let old_id = bucket_id(nums[i - k] as i64);
                buckets.remove(&old_id);
            }
        }
        false
    }
}

#[cfg(test)]
mod test_lc220 {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0));
        assert!(Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2));
        assert!(!Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3));
        assert!(Solution::contains_nearby_almost_duplicate(vec![-1, -1], 1, 0));
        assert!(!Solution::contains_nearby_almost_duplicate(vec![1, 2], 0, 1));
    }
}
