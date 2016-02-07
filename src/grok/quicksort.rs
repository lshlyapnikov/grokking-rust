pub fn sort(xs: &Vec<i32>) -> Vec<i32> {
    if (xs.len()) <= 1 {
         xs.to_owned()
    } else {
    	let mut ys: Vec<i32> = xs.clone();
    	sort_in_place(&mut ys, 0, xs.len());
    	ys
    }
}

// end is not included
fn sort_in_place(xs: &mut Vec<i32>, start: usize, end: usize) {
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
        swap(xs, p, i-1);
        sort_in_place(xs, start, i - 1);
        sort_in_place(xs, i, end);
    }
}

fn pivot(start: usize, end: usize) -> usize {
    (start + (end - 1)) / 2
} 

fn swap(xs: &mut Vec<i32>, i: usize, j: usize) {
    let tmp = xs[i];
    xs[i] = xs[j];
    xs[j] = tmp;
}