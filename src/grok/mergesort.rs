pub fn sort<T: PartialOrd + Clone>(xs: &Vec<T>) -> Vec<T> {
  if (xs.len()) <= 1 {
    xs.to_owned()
  } else if xs.len() == 2 {
    if xs[0] > xs[1] {
      vec![xs[1].clone(), xs[0].clone()]
    } else {
      xs.to_owned()
    }
  } else {
    let mut left = xs.to_owned();
    let right = left.split_off(xs.len() / 2);
    merge_sort_vectors(&sort(&left), &sort(&right))
  }
}

// TODO: can clone() be avoided? Box?
fn merge_sort_vectors<T: PartialOrd + Clone>(xs: &Vec<T>, ys: &Vec<T>) -> Vec<T> {
  let mut zs = Vec::with_capacity(xs.len() + ys.len());
  let mut i = 0;
  let mut j = 0;
  loop {
    if xs[i] <= ys[j] {
      zs.push(xs[i].clone());
      i += 1;
    } else {
      zs.push(ys[j].clone());
      j += 1;
    }

    if i == xs.len() && j == ys.len() {
      break;
    } else if i == xs.len() && j < ys.len() {
      while j < ys.len() {
        zs.push(ys[j].clone());
        j += 1;
      }
      break;
    } else if i < xs.len() && j == ys.len() {
      while i < xs.len() {
        zs.push(xs[i].clone());
        i += 1;
      }
      break;
    }
  }
  zs
}
