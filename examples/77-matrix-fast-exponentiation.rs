fn main() {
    println!("res: {}", Solution::fib(3));
    println!("res: {}", Solution::tribonacci(4));
    // => 1
    // => 1
    // => 2
    // => 5
    // => 11
    // => 24
    // => 53
    // => 117
    // f(n) = 2*f(n-1) = f(n-3)
    for i in 0..10 {
        assert_eq!(Solution::dumb_num_tilings(i), Solution::num_tilings(i))
    }
}

struct Solution;

struct Matrix(usize, usize, Vec<i32>);

impl Matrix {
    fn new() -> Self {
        Self(0, 0, vec![])
    }

    fn new_with(r: usize, c: usize, v: Vec<i32>) -> Self {
        Self(r, c, v)
    }

    fn new_unit(a: usize) -> Self {
        let mut v = vec![0; a * a];
        for i in 0..a {
            v[i * a + i] = 1;
        }
        Self(a, a, v)
    }
}

impl Solution {
    fn matrix_multi(Matrix(r1, c1, v1): &Matrix, Matrix(r2, c2, v2): &Matrix) -> Matrix {
        assert_eq!(c1, r2);
        let mut res = Matrix::new();
        res.0 = *r1;
        res.1 = *c2;
        res.2 = vec![0; r1 * c2];
        for i in 0..*r1 {
            for j in 0..*c2 {
                let mut cur = 0;
                for k in 0..*r2 {
                    cur += v1[i * c1 + k] * v2[k * c2 + j];
                }
                res.2[i * (*c2) + j] = cur;
            }
        }
        res
    }

    fn matrix_exp(mut matrix: Matrix, mut a: i32) -> Matrix {
        assert!(matrix.0 == matrix.1);
        let mut res = Matrix::new_unit(matrix.0);
        while a > 0 {
            if a & 1 != 0 {
                res = Self::matrix_multi(&res, &matrix);
            }
            matrix = Self::matrix_multi(&matrix, &matrix);
            a >>= 1;
        }
        res
    }

    fn matrix_multi_r(Matrix(r1, c1, v1): &Matrix, Matrix(r2, c2, v2): &Matrix, r: i32) -> Matrix {
        assert_eq!(c1, r2);
        let mut res = Matrix::new();
        res.0 = *r1;
        res.1 = *c2;
        res.2 = vec![0; r1 * c2];
        for i in 0..*r1 {
            for j in 0..*c2 {
                let mut cur = 0;
                for k in 0..*r2 {
                    cur = ((cur as i64
                        + (v1[i * c1 + k] as i64 * v2[k * c2 + j] as i64) % r as i64)
                        % r as i64) as i32
                        % r;
                }
                res.2[i * (*c2) + j] = cur;
            }
        }
        res
    }

    fn matrix_exp_r(mut matrix: Matrix, mut a: i32, r: i32) -> Matrix {
        assert!(matrix.0 == matrix.1);
        let mut res = Matrix::new_unit(matrix.0);
        while a > 0 {
            if a & 1 != 0 {
                res = Self::matrix_multi_r(&res, &matrix, r);
            }
            matrix = Self::matrix_multi_r(&matrix, &matrix, r);
            a >>= 1;
        }
        res
    }

    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            n if n > 1 => {
                let rel = Matrix::new_with(2, 2, vec![1, 1, 1, 0]);
                let ini = Matrix::new_with(1, 2, vec![1, 0]);
                let res = Self::matrix_multi(&ini, &Self::matrix_exp(rel, n - 1));
                res.2[0]
            }
            _ => panic!(),
        }
    }

    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            n if n > 2 => {
                let rel = Matrix::new_with(2, 2, vec![1, 1, 1, 0]);
                let ini = Matrix::new_with(1, 2, vec![2, 1]);
                let res = Self::matrix_multi(&ini, &Self::matrix_exp(rel, n - 2));
                res.2[0]
            }
            _ => panic!(),
        }
    }

    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            n if n > 2 => {
                let rel = Matrix::new_with(3, 3, vec![1, 1, 0, 1, 0, 1, 1, 0, 0]);
                let ini = Matrix::new_with(1, 3, vec![1, 1, 0]);
                let res = Self::matrix_multi(&ini, &Self::matrix_exp(rel, n - 2));
                res.2[0]
            }
            _ => panic!(),
        }
    }

    fn dumb_num_tilings(n: i32) -> i32 {
        Self::dnt(n, 0)
    }

    fn dnt(n: i32, i: i32) -> i32 {
        match n {
            0 | 1 => 1,
            2 => 2,
            n if n > 2 => {
                let mut res = 0;
                if i == 0 {
                    res += Self::dnt(n - 2, 0) + Self::dnt(n - 1, 0) + Self::dnt(n - 2, 1) * 2;
                } else {
                    res += Self::dnt(n - 1, 1) + Self::dnt(n - 1, 0)
                }
                res
            }
            _ => panic!(),
        }
    }

    pub fn num_tilings(n: i32) -> i32 {
        match n {
            0 | 1 => 1,
            2 => 2,
            n if n > 2 => {
                let rel = Matrix::new_with(3, 3, vec![2, 1, 0, 0, 0, 1, 1, 0, 0]);
                let ini = Matrix::new_with(1, 3, vec![2, 1, 1]);
                Self::matrix_multi_r(&ini, &Self::matrix_exp_r(rel, n - 2, Self::MOD), Self::MOD).2
                    [0]
            }
            _ => panic!(),
        }
    }

    pub fn count_vowel_permutation(n: i32) -> i32 {
        match n {
            1 => 5,
            n if n > 1 => {
                let rel = Matrix::new_with(
                    5,
                    5,
                    vec![
                        0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0,
                    ],
                );
                let ini = Matrix::new_with(1, 5, vec![1, 1, 1, 1, 1]);
                let res = Self::matrix_multi_r(
                    &ini,
                    &Self::matrix_exp_r(rel, n - 1, Self::MOD),
                    Self::MOD,
                );
                res.2.into_iter().fold(0, |acc, x| {
                    ((acc as i64 + x as i64) % Self::MOD as i64) as i32
                })
            }
            _ => unreachable!(),
        }
    }

    pub fn check_record(n: i32) -> i32 {
        match n {
            1 => 3,
            n if n > 1 => {
                let rel = Matrix::new_with(
                    6,
                    6,
                    vec![
                        1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0,
                        0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0,
                    ],
                );
                let ini = Matrix::new_with(1, 6, vec![1, 1, 0, 1, 0, 0]);
                Self::matrix_multi_r(&ini, &Self::matrix_exp_r(rel, n - 1, Self::MOD), Self::MOD)
                    .2
                    .into_iter()
                    .fold(0, |acc, x| {
                        ((acc as i64 + x as i64) % Self::MOD as i64) as i32
                    })
            }
            _ => unreachable!(),
        }
    }

    const MOD: i32 = 1_000_000_007;
}
