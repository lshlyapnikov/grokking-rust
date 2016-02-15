pub fn sort<T: PartialOrd + Clone>(xs: &Vec<T>) -> Vec<T> {
    let mut ys: Vec<T> = xs.to_vec();
    sort_in_place(&mut ys);
    ys
}

pub fn sort_in_place<T: PartialOrd>(xs: &mut Vec<T>) {
    let size = xs.len();
    if size >= 1 {
        _sort_in_place(xs, 0, size);
    }
}

// end is not included
fn _sort_in_place<T: PartialOrd>(xs: &mut Vec<T>, start: usize, end: usize) {
    let len = end - start;
    if len <= 1 {
        // we are done
    } else if len == 2 {
        if xs[start] > xs[start + 1] {
            swap(xs, start, start + 1);
        }
    } else {
        let mut p: usize = pivot(start, end);
        swap(xs, start, p);
        p = start;
        let mut i = start + 1; // i points to the next element after the pivot ( > pivot)
        let mut j = start + 1; // j points to the 1st unpartitioned element

        while j < end {
            if xs[j] < xs[p] {
                swap(xs, j, i);
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        swap(xs, p, i - 1);
        _sort_in_place(xs, start, i - 1);
        _sort_in_place(xs, i, end);
    }
}

fn pivot(start: usize, end: usize) -> usize {
    (start + (end - 1)) / 2
}

// TODO: implement your own swap, does it have to be unsafe?
fn swap<T>(xs: &mut Vec<T>, i: usize, j: usize) {
    xs.swap(i, j);
}
