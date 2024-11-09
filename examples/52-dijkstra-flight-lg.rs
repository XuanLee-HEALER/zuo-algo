use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut first = false;
    let mut second = false;
    let mut n = 0;
    let mut m = 0;
    let mut k = 0;
    let mut start = 0;
    let mut end = 0;

    let mut head = vec![];
    let mut next = vec![];
    let mut to = vec![];
    let mut weight = vec![];
    let mut cnt = 1;

    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !first {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                k = segs.next().unwrap().parse().unwrap();
                head = vec![0; n + 1];
                next = vec![0; 2 * m + 1];
                to = vec![0; 2 * m + 1];
                weight = vec![0; 2 * m + 1];
                first = true;
            } else if !second {
                start = segs.next().unwrap().parse().unwrap();
                end = segs.next().unwrap().parse().unwrap();
                second = true
            } else {
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                let w: i32 = segs.next().unwrap().parse().unwrap();
                add_edge(
                    &mut cnt,
                    &mut head,
                    &mut next,
                    &mut to,
                    &mut weight,
                    p1,
                    p2,
                    w,
                );
                add_edge(
                    &mut cnt,
                    &mut head,
                    &mut next,
                    &mut to,
                    &mut weight,
                    p2,
                    p1,
                    w,
                );
                m -= 1;

                if m == 0 {
                    let mut distances = vec![vec![i32::MAX; k + 1]; n];
                    let mut point_idx = vec![vec![-1; k + 1]; n];
                    let mut min_heap = vec![(0, 0, 0); n];
                    let mut heap_size = 0;
                    add_or_ignore_or_update(
                        &mut min_heap,
                        &mut point_idx,
                        &mut heap_size,
                        (start, k),
                        0,
                    );

                    while heap_size > 0 {
                        let (city, free, cost) = pop(&mut min_heap, &mut point_idx, &mut heap_size);
                        distances[city][free] = cost as i32;
                        if city == end {
                            bw.write_fmt(format_args!("{}\n", cost)).unwrap();
                            break;
                        }

                        let mut next_edge = head[city];
                        while next_edge > 0 {
                            if free > 0 {
                                add_or_ignore_or_update(
                                    &mut min_heap,
                                    &mut point_idx,
                                    &mut heap_size,
                                    (to[next_edge], free - 1),
                                    cost,
                                );
                            }
                            add_or_ignore_or_update(
                                &mut min_heap,
                                &mut point_idx,
                                &mut heap_size,
                                (to[next_edge], free),
                                cost + weight[next_edge] as usize,
                            );

                            next_edge = next[next_edge];
                        }
                    }

                    break;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn add_or_ignore_or_update(
    heap: &mut [(usize, usize, usize)],
    point_idx: &mut [Vec<i32>],
    heap_size: &mut usize,
    new_point: (usize, usize),
    new_weight: usize,
) {
    let (city, free) = new_point;
    match point_idx[city][free] {
        -1 => {
            point_idx[city][free] = *heap_size as i32;
            *heap_size += 1;
            heap_insert(
                heap,
                point_idx,
                new_point,
                new_weight,
                point_idx[city][free] as usize,
            );
        }
        o if o >= 0 => {
            if new_weight < heap[o as usize].2 {
                heap_insert(heap, point_idx, new_point, new_weight, o as usize);
            }
        }
        _ => (),
    }
}

fn heap_insert(
    heap: &mut [(usize, usize, usize)],
    point_idx: &mut [Vec<i32>],
    point: (usize, usize),
    weight: usize,
    mut loc: usize,
) {
    heap[loc] = (point.0, point.1, weight);
    while loc > 0 {
        let p_idx = (loc - 1) >> 1;
        if heap[loc].2 < heap[p_idx].2 {
            heap.swap(loc, p_idx);
            let tmp = point_idx[heap[loc].0][heap[loc].1];
            point_idx[heap[loc].0][heap[loc].1] = point_idx[heap[p_idx].0][heap[p_idx].1];
            point_idx[heap[p_idx].0][heap[p_idx].1] = tmp;
            loc = p_idx;
        } else {
            break;
        }
    }
}

fn heapify(
    heap: &mut [(usize, usize, usize)],
    point_idx: &mut [Vec<i32>],
    heap_size: usize,
    mut loc: usize,
) {
    while loc < heap_size {
        let l = (loc << 1) + 1;
        if l < heap_size && l + 1 < heap_size {
            let a = if heap[l].2 < heap[l + 1].2 { l } else { l + 1 };
            if heap[loc].2 > heap[a].2 {
                heap.swap(loc, a);
                let tmp = point_idx[heap[loc].0][heap[loc].1];
                point_idx[heap[loc].0][heap[loc].1] = point_idx[heap[a].0][heap[a].1];
                point_idx[heap[a].0][heap[a].1] = tmp;
                // point_idx.swap(heap[loc].0, heap[a].0);
                loc = a;
            } else {
                break;
            }
        } else if l < heap_size && heap[loc].2 > heap[l].2 {
            heap.swap(loc, l);
            let tmp = point_idx[heap[loc].0][heap[loc].1];
            point_idx[heap[loc].0][heap[loc].1] = point_idx[heap[l].0][heap[l].1];
            point_idx[heap[l].0][heap[l].1] = tmp;
            // point_idx.swap(heap[loc].0, heap[l].0);
            loc = l;
        } else {
            break;
        }
    }
}

fn pop(
    heap: &mut [(usize, usize, usize)],
    point_idx: &mut [Vec<i32>],
    heap_size: &mut usize,
) -> (usize, usize, usize) {
    let res = heap[0];
    heap.swap(0, *heap_size - 1);
    let tmp = point_idx[heap[0].0][heap[0].1];
    point_idx[heap[0].0][heap[0].1] = point_idx[heap[*heap_size - 1].0][heap[*heap_size - 1].1];
    point_idx[heap[*heap_size - 1].0][heap[*heap_size - 1].1] = tmp;
    // point_idx.swap(heap[0].0, heap[*heap_size - 1].0);
    // 最后将弹出的元素设置为-2，如果先设置，上一行交换后，留下的元素就成为了-2
    point_idx[res.0][res.1] = -2;
    *heap_size -= 1;
    heapify(heap, point_idx, *heap_size, 0);
    res
}

#[allow(clippy::too_many_arguments)]
fn add_edge(
    cnt: &mut usize,
    head: &mut [usize],
    next: &mut [usize],
    to: &mut [usize],
    weight: &mut [i32],
    p1: usize,
    p2: usize,
    w: i32,
) {
    next[*cnt] = head[p1];
    head[p1] = *cnt;
    to[*cnt] = p2;
    weight[*cnt] = w;
    *cnt += 1;
}
