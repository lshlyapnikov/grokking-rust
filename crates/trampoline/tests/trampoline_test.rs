// use bigint_base10::BigInteger;
// use trampoline::*;

// fn ackermann(m: u128, n: u128) -> Trampoline<u128, u128> {
//     if m == 0 {
//         Trampoline::End(n)
//     } else if n == 0 {
//         Trampoline()

//     }

// }

#[cfg(test)]
mod tests {
    fn ackermann_recursive(m: u128, n: u128) -> u128 {
        if m == 0 {
            n + 1
        } else if n == 0 {
            ackermann_recursive(m - 1, 1)
        } else {
            ackermann_recursive(m - 1, ackermann_recursive(m, n - 1))
        }
    }

    fn ackermann_loop(m: usize, n: usize) -> usize {
        let mut arr = vec![vec![0usize; n + 2]; m + 2];

        for y in 0..(n + 2) {
            arr[0][y] = y + 1;
        }

        for x in 1..(m + 2) {
            arr[x][0] = arr[x - 1][1];
        }

        for x in 1..(m + 1) {
            for y in 1..(n + 1) {
                arr[x][y] = arr[x - 1][arr[x][y - 1]];
            }
        }

        arr[m][n]
    }

    #[test]
    fn test_ackermann_recursive() {
        let sut = ackermann_recursive;
        assert_eq!(sut(0, 0), 1);
        assert_eq!(sut(1, 1), 3);
        assert_eq!(sut(1, 2), 4);
        assert_eq!(sut(2, 2), 7);
        assert_eq!(sut(3, 4), 125);
        assert_eq!(sut(3, 6), 509);
        assert_eq!(sut(3, 6), 509);
        // assert_eq!(sut(4, 1), 65533);
    }

    #[test]
    fn test_ackermann_loop() {
        let sut = ackermann_loop;
        assert_eq!(sut(0, 0), 1);
        assert_eq!(sut(1, 1), 3);
        assert_eq!(sut(1, 2), 4);
        // assert_eq!(sut(2, 2), 7);
        assert_eq!(sut(3, 4), 125);
        assert_eq!(sut(3, 6), 509);
        assert_eq!(sut(3, 6), 509);
        // assert_eq!(sut(4, 1), 65533);
    }
}
