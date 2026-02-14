struct Solution;

impl Solution {
    // 香槟塔模拟过程，1poured=1glass，记录 glass[i][j] 的值为倒入的量，然后逐行模拟
    // 先设 glass[0][0] = poured （poured >= 1）
    // 下一层就会收到上一层每一杯的溢出量 overflow/2，记录新值，逐层计算，直到求解出来
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let query_row = query_row as usize;
        let query_glass = query_glass as usize;
        // glass[j] = 第 i 行第 j 杯收到的总倒入量（可以超过 1）
        let mut glass = vec![0.0f64; query_row + 2];
        glass[0] = poured as f64;
        for i in 0..query_row {
            // 从右往左更新，避免覆盖还没用到的值
            // 下一行有 i+2 个杯子，先清零
            let mut next = vec![0.0f64; query_row + 2];
            for j in 0..=i {
                if glass[j] > 1.0 {
                    let overflow = (glass[j] - 1.0) / 2.0;
                    next[j] += overflow;
                    next[j + 1] += overflow;
                }
            }
            glass = next;
        }
        glass[query_glass].min(1.0)
    }
}

#[cfg(test)]
mod test_lc799 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::champagne_tower(1, 1, 1), 0.0);
        assert_eq!(Solution::champagne_tower(2, 1, 1), 0.5);
        assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1.0);
        assert_eq!(Solution::champagne_tower(0, 0, 0), 0.0);
        assert_eq!(Solution::champagne_tower(1, 0, 0), 1.0);
    }
}
