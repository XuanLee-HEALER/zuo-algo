use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut xhead = false;
    let mut n = 0;
    let mut m = 0;
    let mut s = 0;

    let mut head = vec![];
    let mut next = vec![];
    let mut to = vec![];
    let mut weight = vec![];
    let mut cnt = 1;

    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !xhead {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                s = segs.next().unwrap().parse().unwrap();
                head = vec![0; n + 1];
                next = vec![0; m + 1];
                to = vec![0; m + 1];
                weight = vec![0; m + 1];
                xhead = true;
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
                m -= 1;

                if m == 0 {
                    let mut min_heap = vec![(0, 0); n];
                    let mut distances = vec![0; n + 1];
                    let mut point_idx = vec![-1; n + 1];
                    let mut heap_size = 0;
                    add_or_ignore_or_update(&mut min_heap, &mut point_idx, &mut heap_size, s, 0);

                    while heap_size > 0 {
                        let (p1, w) = pop(&mut min_heap, &mut point_idx, &mut heap_size);
                        distances[p1] = w;
                        let mut next_edge = head[p1];
                        while next_edge > 0 {
                            add_or_ignore_or_update(
                                &mut min_heap,
                                &mut point_idx,
                                &mut heap_size,
                                to[next_edge],
                                w + weight[next_edge],
                            );
                            next_edge = next[next_edge];
                        }
                    }

                    for &dis in &distances[1..n] {
                        bw.write_fmt(format_args!("{} ", dis)).unwrap();
                    }
                    bw.write_fmt(format_args!("{}\n", distances[n])).unwrap();

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
    heap: &mut [(usize, i32)],
    point_idx: &mut [i32],
    heap_size: &mut usize,
    new_point: usize,
    new_weight: i32,
) {
    match point_idx[new_point] {
        -1 => {
            point_idx[new_point] = *heap_size as i32;
            *heap_size += 1;
            heap_insert(
                heap,
                point_idx,
                new_point,
                new_weight,
                point_idx[new_point] as usize,
            );
        }
        o if o >= 0 => {
            if new_weight < heap[o as usize].1 {
                heap_insert(heap, point_idx, new_point, new_weight, o as usize);
            }
        }
        _ => (),
    }
}

fn heap_insert(
    heap: &mut [(usize, i32)],
    point_idx: &mut [i32],
    point: usize,
    weight: i32,
    mut loc: usize,
) {
    heap[loc] = (point, weight);
    while loc > 0 {
        let p_idx = (loc - 1) >> 1;
        if heap[loc].1 < heap[p_idx].1 {
            heap.swap(loc, p_idx);
            point_idx.swap(heap[loc].0, heap[p_idx].0);
            loc = p_idx;
        } else {
            break;
        }
    }
}

fn heapify(heap: &mut [(usize, i32)], point_idx: &mut [i32], heap_size: usize, mut loc: usize) {
    while loc < heap_size {
        let l = (loc << 1) + 1;
        if l < heap_size && l + 1 < heap_size {
            let a = if heap[l].1 < heap[l + 1].1 { l } else { l + 1 };
            if heap[loc].1 > heap[a].1 {
                heap.swap(loc, a);
                point_idx.swap(heap[loc].0, heap[a].0);
                loc = a;
            } else {
                break;
            }
        } else if l < heap_size && heap[loc].1 > heap[l].1 {
            heap.swap(loc, l);
            point_idx.swap(heap[loc].0, heap[l].0);
            loc = l;
        } else {
            break;
        }
    }
}

fn pop(heap: &mut [(usize, i32)], point_idx: &mut [i32], heap_size: &mut usize) -> (usize, i32) {
    let res = heap[0];
    heap.swap(0, *heap_size - 1);
    point_idx.swap(heap[0].0, heap[*heap_size - 1].0);
    // 最后将弹出的元素设置为-2，如果先设置，上一行交换后，留下的元素就成为了-2
    point_idx[res.0] = -2;
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
