pub enum Trampoline<A, R> {
    Continue(A, R),
    End(R),
}

impl<A, R> Trampoline<A, R> {
    pub fn go(f: fn(A, R) -> Trampoline<A, R>, a0: A, acc0: R) -> R {
        let mut a = a0;
        let mut acc = acc0;
        loop {
            match f(a, acc) {
                Trampoline::Continue(a1, acc1) => {
                    a = a1;
                    acc = acc1;
                }
                Trampoline::End(acc1) => return acc1,
            }
        }
    }
}

