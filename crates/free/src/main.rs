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

fn main() {
    let st0 = Free::Pure(0);
    // println!("st0: {:?}", st0);

    let st1: Free<i32> = Free::Suspend(Box::new(|| Free::Pure(100)));
    let st2: Free<i32> = Free::Suspend(Box::new(|| st0));
    // println!("st1: {:?}", st1);

    let sumR: i32 = Free::go(sum(&[1, 2, 3, 6, 7]));
    println!("sum: {:?}", sumR)
}
