#[macro_use]
extern crate log;
extern crate env_logger;
extern crate quickcheck;
use quickcheck::quickcheck;
mod grok;
use grok::mergesort;
use grok::quicksort;
use std::fmt::Debug;


fn main() {
    fn test_sort_function<F, T: PartialOrd + Debug>(sut: F, input: &Vec<T>) -> bool
        where F: Fn(&Vec<T>) -> Vec<T>
    {
        debug!("input :     {:?}", input);
        let sorted: Vec<T> = sut(&input);
        debug!("sorted: {:?}", sorted);
        let mut result = true;
        for win in sorted.windows(2) {
            if win[0] > win[1] {
                result = false;
                break;
            }
        }
        result
    }

    fn mergesort_quickcheck_with_ints(xs: Vec<i32>) -> bool {
        test_sort_function(mergesort::sort, &xs)
    }

    fn mergesort_quickcheck_with_strs(xs: Vec<String>) -> bool {
        test_sort_function(mergesort::sort, &xs)
    }

    fn quicksort_quickcheck_with_ints(xs: Vec<i32>) -> bool {
        test_sort_function(quicksort::sort, &xs)
    }

    fn quicksort_quickcheck_with_strs(xs: Vec<String>) -> bool {
        test_sort_function(quicksort::sort, &xs)
    }

    env_logger::init().unwrap();
    info!("mergesort::sort");
    quickcheck(mergesort_quickcheck_with_ints as fn(Vec<i32>) -> bool);
    quickcheck(mergesort_quickcheck_with_strs as fn(Vec<String>) -> bool);
    info!("quicksort::sort");
    quickcheck(quicksort_quickcheck_with_ints as fn(Vec<i32>) -> bool);
    quickcheck(quicksort_quickcheck_with_strs as fn(Vec<String>) -> bool);
}

#[cfg(test)]
mod tests {
    //    use quickcheck::quickcheck;
    //    use super::mergesort;
    //
    //    #[test]
    //    fn test_mergesort() {
    //        let input = vec![30, 20, 10];
    //        let actual = mergesort(&input);
    //        let expected: Vec<i32> = vec![10, 20, 30];
    //        assert_eq!(actual, expected)
    //    }
    //
    //    #[test]
    //    fn test_mergesort_quickcheck() {
    // 		fn test_sut<F>(sut: F, input: Vec<i32>) -> bool where F: Fn(&Vec<i32>) -> Vec<i32> {
    //    	    debug!("input :     {:?}", input);
    //            let sorted = sut(&input);
    //            debug!("sorted: {:?}", sorted);
    //            let mut result = true;
    //            for win in sorted.windows(2) {
    //                if win[0] > win[1] {
    //                    result = false;
    //                    break;
    //                }
    //            }
    //            result
    //    	}
    //
    // 		fn mergesort_quickcheck(xs: Vec<i32>) -> bool {
    // 			test_sut(mergesort, xs)
    //        }
    //
    //    	quickcheck(mergesort_quickcheck as fn(Vec<i32>) -> bool);
    // 	}
}
