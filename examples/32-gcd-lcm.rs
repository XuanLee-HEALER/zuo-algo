use std::str::FromStr;

use num_bigint::BigInt;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    // for _ in 0..10000 {
    //     let a = rng.gen_range(20..100_000);
    //     let b = rng.gen_range(50_000..1_000_000);
    //     assert_eq!(gcd(a, b), stein(a, b));
    // }
    // assert_eq!(2, nth_magical_number(1, 2, 3));
    // assert_eq!(284734242, nth_magical_number(49384923, 244, 29));
    // (a + b) * (c - d) + (a * b - c * d)
    assert_eq!((5 + 6) % 3, add_same_mod(5, 6, 3));
    assert_eq!((-3 - 12) % 4 + 4, sub_same_mod(-3, 12, 4));
    assert_eq!((13 * 129) % 22, multi_same_mod(13, 129, 22));

    const MOD: i32 = 1_000_000_007;
    for count in 0..10_000 {
        let a = rng.gen_range(0..=i32::MAX);
        let b = rng.gen_range(0..=i32::MAX);
        let c = rng.gen_range(0..=i32::MAX);
        let d = rng.gen_range(0..=i32::MAX);
        let res1 = add_same_mod(
            multi_same_mod(add_same_mod(a, b, MOD), sub_same_mod(c, d, MOD), MOD),
            sub_same_mod(multi_same_mod(a, b, MOD), multi_same_mod(c, d, MOD), MOD),
            MOD,
        );
        let ba = BigInt::from_str(a.to_string().as_str()).unwrap();
        let bb = BigInt::from_str(b.to_string().as_str()).unwrap();
        let bc = BigInt::from_str(c.to_string().as_str()).unwrap();
        let bd = BigInt::from_str(d.to_string().as_str()).unwrap();
        let mut res2 = (((ba.clone() + bb.clone()) * (bc.clone() - bd.clone()))
            + (ba.clone() * bb.clone() - bc.clone() * bd.clone()))
            % MOD;
        res2 = if res2 < BigInt::new(num_bigint::Sign::Plus, vec![0]) {
            res2 + MOD
        } else {
            res2
        };
        assert_eq!(
            res1.to_string(),
            res2.to_string(),
            "current count: {}\na({}) b({}) c({}) d({})\nba({}) bb({}) bc({}) bd({})",
            count,
            a,
            b,
            c,
            d,
            ba,
            bb,
            bc,
            bd
        )
    }
}

fn add_same_mod(a: i32, b: i32, mo: i32) -> i32 {
    (a % mo + b % mo) % mo
}

fn multi_same_mod(a: i32, b: i32, mo: i32) -> i32 {
    i32::try_from((a % mo) as i64 * (b % mo) as i64 % (mo as i64)).unwrap()
}

fn sub_same_mod(a: i32, b: i32, mo: i32) -> i32 {
    ((a % mo - b % mo) + mo) % mo
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn stein(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        match (a % 2, b % 2) {
            (0, 0) => 2 * stein(a / 2, b / 2),
            (1, 0) => stein(a, b / 2),
            (0, 1) => stein(a / 2, b),
            (1, 1) => {
                let (min, max) = if a > b { (b, a) } else { (a, b) };
                stein(min, max - min)
            }
            _ => unreachable!(),
        }
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a / stein(a, b) * b
}

fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    const MOD: i64 = 1_000_000_000 + 7;
    let ab_lcm = lcm(a, b);
    let mut l = 1_i64;
    let mut r = a as i64 * n as i64;
    let mut res = 0;
    while l <= r {
        let mid = l + (r - l) / 2;
        if mid / a as i64 + mid / b as i64 - mid / ab_lcm as i64 >= n as i64 {
            res = mid;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    (res % MOD) as i32
}

// 证明辗转相除法就是证明如下关系：
// gcd(a, b) = gcd(b, a % b)
// 假设a % b = r，即需要证明的关系为：gcd(a, b) = gcd(b, r)
// 证明过程：
// 因为a % b = r，所以如下两个等式必然成立
// 1) a = b * q + r，q为0、1、2、3....中的一个整数
// 2) r = a − b * q，q为0、1、2、3....中的一个整数
// 假设u是a和b的公因子，则有: a = s * u, b = t * u
// 把a和b带入2)得到，r = s * u - t * u * q = (s - t * q) * u
// 这说明 : u如果是a和b的公因子，那么u也是r的因子
// 假设v是b和r的公因子，则有: b = x * v, r = y * v
// 把b和r带入1)得到，a = x * v * q + y * v = (x * q + y) * v
// 这说明 : v如果是b和r的公因子，那么v也是a的公因子
// 综上，a和b的每一个公因子 也是 b和r的一个公因子，反之亦然
// 所以，a和b的全体公因子集合 = b和r的全体公因子集合
// 即gcd(a, b) = gcd(b, r)
// 证明结束
