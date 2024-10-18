use std::cmp::Ordering;

fn main() {
    let matrix = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);
    // println!("result={}", matrix.sum_region(2, 1, 4, 3));
    // println!(
    //     "s result={}",
    //     Solution::largest1_bordered_square(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
    // );
    println!(
        "result={}",
        Solution::field_of_greatest_blessing(vec![
            vec![656366660, 182985354, 8985541],
            vec![905115136, 323596338, 632450388],
            vec![610904014, 299102334, 451967504],
            vec![798426701, 565689705, 646901232],
            vec![555723509, 133598101, 250939748],
            vec![209010168, 599043227, 304180994],
            vec![172145673, 718103814, 372387490],
            vec![899211522, 727758874, 550953663],
            vec![8077495, 362256058, 336984017],
            vec![286461698, 144180, 507774474],
            vec![114865417, 155100262, 675260295],
            vec![901695625, 1633651, 915822416],
            vec![413276132, 100040346, 819630169],
            vec![760161289, 364324155, 791884111],
            vec![276056323, 808619988, 502035337],
            vec![212428686, 362642624, 945523918],
            vec![520546836, 485690698, 799465897],
            vec![945120178, 909334377, 770763399],
            vec![505551333, 213026736, 970403332],
            vec![284261557, 260662838, 271952816],
            vec![553394016, 402101338, 369997273],
            vec![433961678, 946536099, 292933193],
            vec![142495958, 718821393, 832581212],
            vec![653561778, 613642155, 175887938],
            vec![534088615, 495235579, 200258616],
            vec![287904851, 280813117, 943864183],
            vec![937922591, 586111692, 238629139],
            vec![184670028, 183832489, 966456553],
            vec![745159026, 482889037, 496362898],
            vec![16689566, 429843763, 347424611],
            vec![223068369, 15949547, 690071140],
            vec![363878615, 110868064, 24936195],
            vec![281013254, 5084146, 399769209],
            vec![760745404, 115039035, 509240925],
            vec![759093951, 353937508, 667759365],
            vec![924224161, 778159344, 846515986],
            vec![874223802, 223042506, 754793836],
            vec![117325563, 335511654, 307429957],
            vec![706039229, 584945625, 725423969],
            vec![33503710, 123521173, 265423616],
            vec![468234117, 537966604, 22564543],
            vec![333006793, 835565658, 109431065],
            vec![346388268, 323452311, 493153263],
            vec![667904428, 954362748, 782989622],
            vec![773707479, 99346442, 555727041],
            vec![988279332, 109904822, 878912595],
            vec![206736588, 584929736, 967690548],
            vec![354789570, 661976689, 195654591],
            vec![440560805, 309977972, 212316460],
            vec![836833896, 795093328, 876581527],
            vec![999962882, 65418794, 417334718],
            vec![136920004, 263962939, 751634799],
            vec![688239048, 988971610, 646000349],
            vec![133064091, 898366372, 356237167],
            vec![268396285, 119468467, 592405018],
            vec![383852194, 734452741, 142696165],
            vec![817308950, 176685009, 35350108],
            vec![774190025, 477353192, 527791712],
            vec![979387690, 177783797, 444467939],
            vec![507212914, 177935795, 957850991],
            vec![473956758, 499548805, 501432484],
            vec![186580071, 276845848, 850472570],
            vec![737895084, 970913886, 941800230],
            vec![899623062, 890634655, 473371099],
            vec![380144987, 745476952, 394854975],
            vec![459226090, 180182281, 12789307],
            vec![911684716, 472292965, 211954080],
            vec![468538648, 756594293, 608919112],
            vec![440377754, 893079704, 219075565],
            vec![391519262, 613044931, 73144249],
            vec![627148834, 29262958, 526513637],
            vec![113352389, 602066362, 141223824],
            vec![94271348, 406425412, 708339425],
            vec![598758369, 17535996, 94224463],
            vec![859847947, 131749505, 772116423],
            vec![486222779, 107851335, 557284891],
            vec![140679879, 332650122, 744626195],
            vec![103143910, 88974938, 721327052],
            vec![16699953, 676810867, 594799719],
            vec![862955100, 825202336, 434808859],
            vec![563606772, 409563164, 658674668],
            vec![922060493, 224047205, 299510131],
            vec![752468950, 275406026, 223190415],
            vec![414739726, 776627070, 84519288],
            vec![702852270, 412145950, 459640188],
            vec![515401876, 246452828, 24503898],
            vec![254953067, 497266753, 584710197],
            vec![422309843, 692705618, 512363299],
            vec![684049430, 638428839, 770346853],
            vec![773977339, 541894771, 798543656],
            vec![103754377, 601024254, 368003523],
            vec![452965377, 898657937, 793754192],
            vec![61512955, 395156338, 982094527],
            vec![725111202, 200330863, 3086947],
            vec![902923805, 456836634, 212261090],
            vec![318182471, 686977590, 662133629],
            vec![645084815, 865692805, 316931246]
        ])
    );
}

struct Solution;

impl Solution {
    pub fn field_of_greatest_blessing(force_field: Vec<Vec<i32>>) -> i32 {
        let mut xrange = Vec::new();
        let mut yrange = Vec::new();
        for field in &force_field {
            let (x, y, side) = (field[0] as i64, field[1] as i64, field[2] as i64);
            xrange.push((x << 1) - side);
            xrange.push((x << 1) + side);
            yrange.push((y << 1) - side);
            yrange.push((y << 1) + side);
        }

        xrange.sort();
        yrange.sort();
        let mut col = 1;
        let mut row = 1;
        for i in 1..xrange.len() {
            if xrange[i] != xrange[i - 1] {
                xrange[col] = xrange[i];
                col += 1;
            }
        }

        for i in 1..yrange.len() {
            if yrange[i] != yrange[i - 1] {
                yrange[row] = yrange[i];
                row += 1;
            }
        }

        let bin_find = |arr: &[i64], v: i64, l: usize| -> Option<usize> {
            let mut left = 0;
            let mut right = l - 1;
            while left <= right {
                let mid = left + ((right - left) >> 1);
                match arr[mid].cmp(&v) {
                    Ordering::Greater => {
                        right = if mid == 0 {
                            break;
                        } else {
                            mid - 1
                        }
                    }
                    Ordering::Less => left = mid + 1,
                    Ordering::Equal => return Some(mid),
                }
            }
            None
        };

        let mut dif_arr = vec![vec![0; col + 2]; row + 2];
        for field in &force_field {
            let (x, y, side) = (field[0] as i64, field[1] as i64, field[2] as i64);
            let r2 = bin_find(&yrange, (y << 1) + side, row).unwrap();
            let c2 = bin_find(&xrange, (x << 1) + side, col).unwrap();
            let r1 = bin_find(&yrange, (y << 1) - side, row).unwrap();
            let c1 = bin_find(&xrange, (x << 1) - side, col).unwrap();
            dif_arr[r1 + 1][c1 + 1] += 1;
            dif_arr[r2 + 2][c1 + 1] -= 1;
            dif_arr[r1 + 1][c2 + 2] -= 1;
            dif_arr[r2 + 2][c2 + 2] += 1;
        }

        let mut ans = 0;
        for i in 1..=row {
            for j in 1..=col {
                dif_arr[i][j] += dif_arr[i - 1][j] + dif_arr[i][j - 1] - dif_arr[i - 1][j - 1];
                ans = ans.max(dif_arr[i][j]);
            }
        }

        ans
    }

    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let row = grid.len();
        let col = grid[0].len();

        let pre_sum_arr = Self::pre_sum(&grid, row, col);
        let mut new_vec = vec![vec![0; col + 2]; row + 2];

        for (i, sub) in grid.iter().enumerate() {
            for (j, e) in sub.iter().enumerate() {
                if *e == 0 {
                    let r1 = i;
                    let c1 = j;
                    let r2 = r1 + stamp_height as usize - 1;
                    let c2 = c1 + stamp_width as usize - 1;
                    if r2 >= row || c2 >= col {
                        continue;
                    } else if pre_sum_arr[r2 + 1][c2 + 1]
                        - pre_sum_arr[r2 + 1][c1]
                        - pre_sum_arr[r1][c2 + 1]
                        + pre_sum_arr[r1][c1]
                        == 0
                    {
                        Self::set(&mut new_vec, r1 + 1, c1 + 1, r2 + 1, c2 + 1);
                    }
                }
            }
        }

        Self::build(&mut new_vec, row, col);

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 0 && new_vec[i + 1][j + 1] == 0 {
                    return false;
                }
            }
        }

        true
    }

    fn set(arr: &mut [Vec<i32>], r1: usize, c1: usize, r2: usize, c2: usize) {
        arr[r1][c1] += 1;
        arr[r2 + 1][c1] -= 1;
        arr[r1][c2 + 1] -= 1;
        arr[r2 + 1][c2 + 1] += 1;
    }

    fn build(arr: &mut [Vec<i32>], row: usize, col: usize) {
        for i in 1..=row {
            for j in 1..=col {
                arr[i][j] += arr[i - 1][j] + arr[i][j - 1] - arr[i - 1][j - 1]
            }
        }
    }

    fn pre_sum(arr: &[Vec<i32>], row: usize, col: usize) -> Vec<Vec<i32>> {
        let mut new_vec = vec![vec![0; col + 1]; row + 1];
        for i in 0..row {
            for j in 0..col {
                new_vec[i + 1][j + 1] = arr[i][j];
                new_vec[i + 1][j + 1] += new_vec[i + 1][j] + new_vec[i][j + 1] - new_vec[i][j];
            }
        }
        new_vec
    }

    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        Self::sum_arr(&mut grid);
        let row = grid.len();
        let col = grid[0].len();
        if grid[row - 1][col - 1] == 0 {
            return 0;
        }

        let mut ans: i32 = 1;
        for i in 0..row {
            for j in 0..col {
                let mut k = ans;
                loop {
                    let ni = i + k as usize;
                    let nj = j + k as usize;
                    if ni >= row || nj >= col {
                        break;
                    } else if (Self::sum(&grid, i, j, ni, nj)
                        - Self::sum(&grid, i + 1, j + 1, ni - 1, nj - 1))
                        == k << 2
                    {
                        ans = k + 1;
                    }
                    k += 1;
                }
            }
        }

        ans * ans
    }

    fn sum_arr(arr: &mut [Vec<i32>]) {
        for i in 0..arr.len() {
            for j in 0..arr[i].len() {
                arr[i][j] += if i == 0 { 0 } else { arr[i - 1][j] }
                    + if j == 0 { 0 } else { arr[i][j - 1] }
                    - if i == 0 || j == 0 {
                        0
                    } else {
                        arr[i - 1][j - 1]
                    };
            }
        }
    }

    fn sum(arr: &[Vec<i32>], r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
        if r1 > r2 {
            0
        } else {
            let r2c2 = arr[r2][c2];
            let l2l = if c1 == 0 { 0 } else { arr[r2][c1 - 1] };
            let r1u = if r1 == 0 { 0 } else { arr[r1 - 1][c2] };
            let r1c1lu = if r1 == 0 || c1 == 0 {
                0
            } else {
                arr[r1 - 1][c1 - 1]
            };
            r2c2 - l2l - r1u + r1c1lu
        }
    }
}

struct NumMatrix {
    remix_arr: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut new_matrix = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for (i, sub) in matrix.iter().enumerate() {
            for (j, e) in sub.iter().enumerate() {
                new_matrix[i + 1][j + 1] = *e;
            }
        }

        for i in 1..new_matrix.len() {
            for j in 1..new_matrix[i].len() {
                new_matrix[i][j] +=
                    new_matrix[i - 1][j] + new_matrix[i][j - 1] - new_matrix[i - 1][j - 1];
            }
        }
        // println!("{:?}", new_matrix);
        Self {
            remix_arr: new_matrix,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, row2, col1, col2) = (row1 as usize, row2 as usize, col1 as usize, col2 as usize);
        self.remix_arr[row2 + 1][col2 + 1]
            - self.remix_arr[row2 + 1][col1]
            - self.remix_arr[row1][col2 + 1]
            + self.remix_arr[row1][col1]
    }
}
