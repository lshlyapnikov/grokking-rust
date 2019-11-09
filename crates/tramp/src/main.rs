use std::fmt::Debug;

use bigint_base10::BigInteger;

fn factorial_rec(n: BigInteger) -> BigInteger {
    if n > BigInteger::new("1") {
        let x: BigInteger = n.clone() - &BigInteger::new("1");
        let y: BigInteger = factorial_rec(x);
        n * &y
    } else {
        BigInteger::new("1")
    }
}

fn factorial_so1(n: u128) -> u128 {
    if n > 1 {
        n + factorial_so1(n - 1)
    } else {
        1
    }
}

fn factorial_so(n: u128) -> u128 {
    fn factorial_loop(n: u128, acc: u128) -> u128 {
        if n > 1 {
            factorial_loop(n - 1, n + acc)
        } else {
            acc
        }
    }

    factorial_loop(n, 1)
}

enum Trampoline<A, R> {
    Continue(A, R),
    End(R),
}

impl<A: Debug, R: Debug> Trampoline<A, R> {
    fn go(f: fn(A, R) -> Trampoline<A, R>, a0: A, acc0: R) -> R {
        let mut a = a0;
        let mut acc = acc0;
        loop {
            // println!("a: {:?}, acc: {:?}", a, acc);
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

fn factorial_trampoline(n: u128, acc: u128) -> Trampoline<u128, u128> {
    if n > 1 {
        Trampoline::Continue(n - 1, n + acc)
    } else {
        Trampoline::End(acc)
    }
}

fn factorial_tr(n: u128) -> u128 {
    Trampoline::go(factorial_trampoline, n, 1)
}

fn main() {
    // println!("factorial_rec: {:?}", factorial_rec(BigInteger::new("12345")));
    println!("factorial_so() = {:?}", factorial_so(12345));
    println!("factorial_tr() = {:?}", factorial_tr(12345678));
    // println!("factorial_so1() = {:?}", factorial_so1(123457));
    // println!("factorial_tr({:?}) = {:?}", 12, factorial_tr(12));

    // factorial_tr(128);
}
