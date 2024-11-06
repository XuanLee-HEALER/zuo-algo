use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        Self::candy_2(&ratings)
    }

    /// greedy 局部最优导向全局最优
    ///
    /// 第一步，每个孩子至少一个糖果
    ///
    /// 第二步，从左往右遍历，每个孩子要比自己左边的孩子多一个糖果，此时每个孩子比自己左边孩子糖果多条件满足，本次遍历左边孩子的糖果数已经解决
    ///
    /// 第三步，从右往左遍历，每个孩子要比自己右边的孩子多一个糖果，此时它当前的糖果是满足左边条件的，要满足右边条件需要`max(cur_candy, next_child_candy+1)`，本地遍历右边孩子的糖果数已经解决
    fn candy_1(ratings: &[i32]) -> i32 {
        let mut res = vec![1; ratings.len()];

        // 第一次，满足每个人都比自己左边的低级人糖果多
        for (idx, pair) in ratings.windows(2).enumerate() {
            if pair[1] > pair[0] {
                res[idx + 1] = res[idx] + 1;
            }
        }

        // 第二次，满足每个人都至少比自己右边的低级人糖果多且同时满足左边条件
        for (idx, pair) in ratings.windows(2).rev().enumerate() {
            if pair[0] > pair[1] {
                res[ratings.len() - idx - 2] =
                    (res[ratings.len() - idx - 1] + 1).max(res[ratings.len() - idx - 2]);
            }
        }

        res.iter().sum()
    }

    /// up down peak
    ///
    /// 记录孩子权重的上升趋势，权重峰值和下降趋势
    ///
    /// 如果当前孩子对于上一个孩子是上升趋势，那么上升趋势累加1，它需要的糖果数是上升趋势+1，持续记录峰值（上升趋势）。如果趋势平缓，那么变化停止，当前孩子只需要1个糖果。如果趋势下降，上升趋势为0，当前孩子至少需要1个糖果，且上一个孩子多给**下降趋势**的糖果，如果下降次数小于等于上升最高次数，那么之前得到最多糖果的孩子可以减少1个糖果，也不影响满足要求
    ///
    /// 如果权重趋势平缓，则在平缓的孩子位置之前的糖果数不再影响后续的糖果分发
    ///
    /// 如果权重趋势先增后减，就需要按照增减逻辑计算
    ///
    /// 如果权重趋势先减后增，那么前面下降趋势不影响后面升高趋势的计算
    fn candy_2(ratings: &[i32]) -> i32 {
        // 如果上升，记录加1，如果不动，记录，如果降低，峰值可以掉n个
        let mut up = 0;
        let mut down = 0;
        let mut peak = 0;
        let mut ret = 1;

        for pair in ratings.windows(2) {
            let (x, y) = (pair[0], pair[1]);
            match x.cmp(&y) {
                Ordering::Less => {
                    up += 1;
                    peak = up;
                    down = 0;
                    ret += up + 1;
                }
                Ordering::Equal => {
                    up = 0;
                    down = 0;
                    peak = 0;
                    ret += 1;
                }
                Ordering::Greater => {
                    up = 0;
                    down += 1;
                    ret += down + 1;
                    if peak >= down {
                        ret -= 1;
                    }
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod test_lc135 {
    #[test]
    fn test_lc135() {
        // 18
        let v1 = vec![8, 6, 2, 1, 9, 3, 5, 4, 7];
        // 16
        let v2 = vec![2, 6, 3, 7, 8, 1, 5, 9, 4];

        assert_eq!(18, super::Solution::candy(v1));
        assert_eq!(16, super::Solution::candy(v2));
    }

    #[test]
    fn test_lc135_1() {
        // 18
        let v1 = vec![8, 6, 2, 1, 9, 3, 5, 4, 7];
        // 16
        let v2 = vec![2, 6, 3, 7, 8, 1, 5, 9, 4];

        assert_eq!(18, super::Solution::candy_2(&v1));
        assert_eq!(16, super::Solution::candy_2(&v2));
    }
}
