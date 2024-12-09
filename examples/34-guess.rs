fn main() {
    // let mut rng = thread_rng();
    // let mut init_arr = (1..=3).collect::<Vec<i32>>();
    // permutation(&mut init_arr, 0);
}

struct Solution;

impl Solution {
    pub fn superpalindromes_in_range_1(left: String, right: String) -> i32 {
        let res: [i64; 86] = [
            121,
            1,
            484,
            4,
            9,
            1002001,
            10201,
            1234321,
            12321,
            14641,
            4008004,
            40804,
            44944,
            10000200001,
            100020001,
            10221412201,
            102030201,
            104060401,
            12102420121,
            121242121,
            12345654321,
            123454321,
            125686521,
            40000800004,
            400080004,
            404090404,
            100000020000001,
            1000002000001,
            100220141022001,
            1002003002001,
            1004006004001,
            102012040210201,
            1020304030201,
            102234363432201,
            1022325232201,
            1024348434201,
            121000242000121,
            1210024200121,
            121242363242121,
            1212225222121,
            1214428244121,
            123212464212321,
            1232346432321,
            123456787654321,
            1234567654321,
            400000080000004,
            4000008000004,
            4004009004004,
            1000000002000000001,
            10000000200000001,
            1000220014100220001,
            10002000300020001,
            10004000600040001,
            1002003004003002001,
            10020210401202001,
            1002223236323222001,
            10022212521222001,
            10024214841242001,
            1020100204020010201,
            10201020402010201,
            1020322416142230201,
            10203040504030201,
            10205060806050201,
            1022123226223212201,
            10221432623412201,
            1022345658565432201,
            10223454745432201,
            1210000024200000121,
            12100002420000121,
            1210242036302420121,
            12102202520220121,
            12104402820440121,
            1212203226223022121,
            12122232623222121,
            1212445458545442121,
            12124434743442121,
            1232100246420012321,
            12321024642012321,
            1232344458544432321,
            12323244744232321,
            1234323468643234321,
            12343456865434321,
            12345678987654321,
            4000000008000000004,
            40000000800000004,
            40004000900040004,
        ];
        let left = left.parse().unwrap();
        let right = right.parse().unwrap();
        res.iter().filter(|&i| *i >= left && *i <= right).count() as i32
    }

    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let left: i64 = left.parse().unwrap();
        let right: i64 = right.parse().unwrap();
        let right_limit = (right as f64).sqrt().floor() as i64;
        let mut seed = 1;
        let mut num = 0;
        let mut res = 0;
        while num <= right_limit {
            num = Self::even_palindrome(seed);
            let t = num.pow(2);
            if t >= left && t <= right && Self::is_palindrome(t) {
                println!("{t},");
                res += 1;
            }

            num = Self::odd_palindrome(seed);
            let t = num.pow(2);
            if t >= left && t <= right && Self::is_palindrome(t) {
                println!("{t},");
                res += 1;
            }

            seed += 1;
        }

        res
    }

    fn even_palindrome(i: i64) -> i64 {
        let mut res = i;
        let mut i = i;
        while i > 0 {
            res = res * 10 + i % 10;
            i /= 10;
        }
        res
    }

    fn odd_palindrome(i: i64) -> i64 {
        let mut res = i;
        let mut i = i / 10;
        while i > 0 {
            res = res * 10 + i % 10;
            i /= 10;
        }
        res
    }

    pub fn is_palindrome(i: i64) -> bool {
        if i < 0 {
            false
        } else {
            let mut base = 1;
            let mut ti = i;
            while ti / 10 > 0 {
                base *= 10;
                ti /= 10;
            }

            let mut i = i;
            while i > 0 {
                if i / base != i % 10 {
                    return false;
                }
                i = i % base / 10;
                base /= 100;
            }

            true
        }
    }

    fn num_len(i: i64) -> usize {
        if i == 0 {
            return 1;
        }
        let mut i = i;
        let mut res = 0;
        while i > 0 {
            res += 1;
            i /= 10;
        }
        res
    }
}

fn permutation(arr: &mut [i32], cur_idx: usize) {
    if cur_idx == arr.len() - 1 {
        println!("{:?}", arr)
    } else {
        for i in cur_idx..arr.len() {
            arr.swap(cur_idx, i);
            permutation(arr, cur_idx + 1);
            arr.swap(cur_idx, i);
        }
    }
}

#[cfg(test)]
mod solution_test {
    #[test]
    fn check_is_palindrome() {
        assert!(super::Solution::is_palindrome(0));
        assert!(super::Solution::is_palindrome(121));
        assert!(!super::Solution::is_palindrome(1000021));
    }

    #[test]
    fn check_even_odd_palindrome() {
        assert_eq!(11211, super::Solution::odd_palindrome(112));
        assert_eq!(221122, super::Solution::even_palindrome(221));
    }

    #[test]
    fn check_super_palindrome() {
        assert_eq!(
            1,
            super::Solution::superpalindromes_in_range("1".into(), i64::MAX.to_string())
        );
        // assert_eq!(
        //     4,
        //     super::Solution::superpalindromes_in_range("4".into(), "1000".into())
        // );
        // assert_eq!(
        //     1,
        //     super::Solution::superpalindromes_in_range("1".into(), "2".into())
        // );
        // assert_eq!(
        //     2,
        //     super::Solution::superpalindromes_in_range("1".into(), "5".into())
        // );
    }
}
