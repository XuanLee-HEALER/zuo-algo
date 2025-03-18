use std::usize;

fn main() {
    // for i in 2..100 {
    //     if Solution::is_prime(i) {
    //         println!("{} is a prime number", i);
    //     }
    // }
    println!("842 -> {:?}", Solution::prime_factorization(842));
    println!(
        "421 is a prime? {}",
        if Solution::is_prime(421) { "Yes" } else { "No" }
    );
    println!(
        "res {}",
        Solution::largest_component_size(vec![4, 6, 15, 35])
    );
    assert_eq!(1229, Solution::count_primes(10000))
}

struct Solution;

impl Solution {
    fn is_prime(n: i32) -> bool {
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    fn prime_factorization(mut n: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut beg = 2;
        while beg * beg <= n {
            if n % beg == 0 {
                res.push(beg);
                while n % beg == 0 {
                    n /= beg;
                }
            }
            beg += 1;
        }
        if n != 1 {
            res.push(n);
        }
        res
    }

    fn find(father: &mut [i32], v: i32) -> i32 {
        if father[v as usize] != v {
            father[v as usize] = Self::find(father, father[v as usize]);
        }
        father[v as usize]
    }

    fn unions(father: &mut [i32], size: &mut [i32], v1: i32, v2: i32) {
        let b1 = Self::find(father, v1);
        let b2 = Self::find(father, v2);
        if b1 != b2 {
            father[b1 as usize] = b2;
            size[b2 as usize] += size[b1 as usize];
        }
    }

    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        const MAXN: usize = 10;
        const MAXM: usize = 100_001;
        let mut fathers = Vec::with_capacity(MAXN);
        for i in 0..MAXN {
            fathers.push(i as i32);
        }
        let mut sizes = vec![1; MAXN];
        let mut history = vec![-1; MAXM];
        for (i, v) in nums.into_iter().enumerate() {
            let segs = Self::prime_factorization(v);
            for seg in segs {
                if history[seg as usize] != -1 {
                    Self::unions(&mut fathers, &mut sizes, i as i32, history[seg as usize]);
                }
                history[seg as usize] = i as i32;
            }
        }
        *sizes.iter().max().unwrap()
    }

    fn ehrlich(n: i32) -> i32 {
        // 记录所有数字是否为质数，true是质数，false不是质数
        let mut is_prime = vec![true; n as usize + 1];
        // 从2开始遍历
        let mut i = 2;
        // 遍历到sqrt(n)即可
        while i * i <= n {
            // 如果i是质数
            if is_prime[i as usize] {
                let mut j = i;
                while j * i <= n {
                    is_prime[(j * i) as usize] = false;
                    j += 1;
                }
            }
            i += 1;
        }
        is_prime.into_iter().skip(2).filter(|v| *v).count() as i32
    }

    fn ehrlich_1(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let mut is_prime = vec![true; n as usize + 1];
        let mut ct = (n + 1) / 2;
        let mut i = 3;
        while i * i <= n {
            if is_prime[i as usize] {
                let mut j = i * i;
                while j <= n {
                    if is_prime[j as usize] {
                        is_prime[j as usize] = false;
                        ct -= 1;
                    }
                    j += 2 * i;
                }
            }
            i += 2;
        }
        ct
    }

    fn euler(n: i32) -> i32 {
        let mut is_prime = vec![true; n as usize + 1];
        let mut primes = Vec::with_capacity((n as usize + 1) / 2);
        let mut i = 2;
        while i <= n {
            if is_prime[i as usize] {
                primes.push(i);
            }
            for &p in &primes {
                if i * p <= n {
                    is_prime[(i * p) as usize] = false;
                } else {
                    break;
                }
                if i % p == 0 {
                    break;
                }
            }
            i += 1;
        }
        primes.len() as i32
    }

    pub fn count_primes(n: i32) -> i32 {
        Self::ehrlich_1(n - 1)
    }
}
