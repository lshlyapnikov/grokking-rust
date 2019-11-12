enum Free<A> {
    Pure(A),
    Suspend(Box<dyn FnOnce() -> Free<A>>),
}

impl<A> Free<A> {
    pub fn go(fa: Free<A>) -> A {
        let mut run: Free<A> = fa;
        loop {
            match run {
                Free::Suspend(ga) => {
                    run = ga();
                }
                Free::Pure(a) => return a,
            }
        }
    }
}

fn sum(arr: &'static [i32]) -> Free<i32> {
    fn go(arr: &'static [i32], acc: i32) -> Free<i32> {
        match arr.first() {
            Some(a) => Free::Suspend(Box::new(move || go(&arr[1..], a + acc))),
            None => Free::Pure(acc.clone()),
        }
    }
    go(arr, 0)
}

fn ackermann(m: u128, n: u128) -> Free<u128> {
    if m == 0 {
        Free::Pure(n + 1)
    } else if n == 0 {
        Free::Suspend(Box::new(move || ackermann(m - 1, 1)))
    } else {
        let a: u128 = Free::go(ackermann(m, n - 1));
        Free::Suspend(Box::new(move || ackermann(m - 1, a)))
    }
}

fn main() {
    let st0 = Free::Pure(0);
    // println!("st0: {:?}", st0);

    let st1: Free<i32> = Free::Suspend(Box::new(|| Free::Pure(100)));
    let st2: Free<i32> = Free::Suspend(Box::new(|| st0));
    // println!("st1: {:?}", st1);

    let sum_r: i32 = Free::go(sum(&[1, 2, 3, 6, 7]));
    println!("sum: {:?}", sum_r);

    println!("ackermann(4, 1): {:?}", Free::go(ackermann(4, 1)));
}
