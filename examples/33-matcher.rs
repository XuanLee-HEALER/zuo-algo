fn main() {
    // for apple in 1..140 {
    //     assert_eq!(bags(apple), bags1(apple))
    // }
    // for grass in 1..60 {
    //     assert_eq!(
    //         eat_grass(grass, "A"),
    //         eat_grass1(grass, "A"),
    //         "grass: {}",
    //         grass
    //     )
    // }
    // for i in 1..1000 {
    //     assert_eq!(is_1(i), is_1_1(i))
    // }
    for i in 1..15 {
        let mut vc = vec![char::default(); i];
        // println!("length {}, result {}", i, a_str(&mut vc, 0))
        assert_eq!(a_str(&mut vc, 0), a_str_1(i as i32))
    }
}

fn a_str(path: &mut Vec<char>, idx: usize) -> i32 {
    if idx == path.len() {
        let mut count = 0;
        for i in 0..path.len() {
            for j in i + 1..path.len() {
                if is_palindrome(path, i, j) {
                    count += 1;
                    if count > 1 {
                        return 0;
                    }
                }
            }
        }
        if count == 1 {
            1
        } else {
            0
        }
    } else {
        path[idx] = 'R';
        let r1 = a_str(path, idx + 1);
        path[idx] = 'E';
        let r2 = a_str(path, idx + 1);
        path[idx] = 'D';
        let r3 = a_str(path, idx + 1);
        r1 + r2 + r3
    }
}

fn a_str_1(n: i32) -> i32 {
    match n {
        1 => 0,
        2 => 3,
        3 => 18,
        n if n >= 4 => (((n as i64 + 1) * 6) % 1_000_000_007_i64) as i32,
        _ => unreachable!(),
    }
}

fn is_palindrome(chars: &[char], mut l: usize, mut r: usize) -> bool {
    while l <= r {
        if chars[l] != chars[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

fn is_1(n: i32) -> bool {
    let mut i = 1;
    while i < n {
        let mut count = 0;
        let mut sum = 0;
        let mut j = i;
        while sum < n {
            sum += j;
            count += 1;
            j += 1;
        }
        if sum == n && count > 1 {
            return true;
        }
        i += 1;
    }
    false
}

fn is_1_1(n: i32) -> bool {
    (n & (n - 1)) != 0
}

fn eat_grass(grass: i32, first: &str) -> &str {
    let enemy = if first == "A" { "B" } else { "A" };
    if grass == 0 {
        enemy
    } else {
        let mut cur_grass = 1;
        while cur_grass <= grass {
            let if_res = eat_grass(grass - cur_grass, enemy);
            if if_res == first {
                return first;
            }
            cur_grass *= 4;
        }
        enemy
    }
}

fn eat_grass1(grass: i32, first: &str) -> &str {
    // a b a a b
    match grass % 5 {
        1 | 3 | 4 => first,
        0 | 2 => {
            if first == "A" {
                "B"
            } else {
                "A"
            }
        }
        _ => unreachable!(),
    }
}

fn bags(n: i32) -> i32 {
    if n == 6 || n == 8 {
        1
    } else if n < 6 {
        -1
    } else {
        let res1 = bags(n - 6);
        let res1 = if res1 == -1 { -1 } else { res1 + 1 };
        let res2 = bags(n - 8);
        let res2 = if res2 == -1 { -1 } else { res2 + 1 };
        if res1 == -1 && res2 == -1 {
            -1
        } else if res1 == -1 {
            res2
        } else if res2 == -1 {
            res1
        } else {
            res1.min(res2)
        }
    }
}

fn bags1(n: i32) -> i32 {
    // 6 8
    // 12 14 16
    if n == 6 || n == 8 {
        1
    } else if n == 12 || n == 14 || n == 16 {
        2
    } else if n < 18 {
        -1
    } else if (n & (1)) != 1 {
        (n - 18) / 8 + 3
    } else {
        -1
    }
}
