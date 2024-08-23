use rand::{thread_rng, Rng};

fn main() {
    let test_count = 100;
    let test_arr_length = 200;
    let test_arr_ele_max = 5000;

    println!("测试开始");
    for _ in 0..test_count {
        let arr = random_array(test_arr_length, test_arr_ele_max);

        let mut arr1 = arr.clone();
        let mut arr2 = arr.clone();
        let mut arr3 = arr.clone();
        selection_sort(&mut arr1);
        bubble_sort(&mut arr2);
        insert_sort(&mut arr3);

        if arr1 == arr2 || arr2 == arr3 {
            println!("测试通过")
        } else {
            println!("测试不通过");
            break;
        }
    }
    println!("测试结束");
}

fn random_array(n: usize, e: i32) -> Vec<i32> {
    let mut res = Vec::<i32>::with_capacity(n);
    let mut rng = thread_rng();
    for _ in 0..n {
        res.push(rng.gen_range(0..=e));
    }
    res
}

fn selection_sort(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let mut i = 0;
        while i < arr.len() {
            let mut min = i;
            let mut j = i;
            while j < arr.len() {
                if arr.get(j).unwrap() < arr.get(min).unwrap() {
                    min = j;
                }
                j += 1;
            }
            arr.swap(i, min);
            i += 1;
        }
    }
}

fn bubble_sort(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let mut end = arr.len() - 1;
        while end > 0 {
            let mut i = 0;
            let mut j = 1;
            while j <= end {
                let prev = arr.get(i).unwrap();
                let next = arr.get(j).unwrap();
                if prev > next {
                    arr.swap(i, j);
                }
                i += 1;
                j += 1;
            }
            end -= 1;
        }
    }
}

fn insert_sort(arr: &mut [i32]) {
    if arr.len() >= 2 {
        // let mut foreach_count = 0;

        for i in 1..arr.len() {
            // foreach_count += 1;
            for j in (0..=i - 1).rev() {
                // foreach_count += 1;
                let next_val = arr.get(j + 1).unwrap();
                let prev_val = arr.get(j).unwrap();
                if next_val < prev_val {
                    arr.swap(j, j + 1);
                }
            }
        }

        // println!("for count => {}", foreach_count)
    }
}
