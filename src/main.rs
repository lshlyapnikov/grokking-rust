#[macro_use]
extern crate log;
extern crate env_logger;
extern crate quickcheck;
mod grok;
use quickcheck::quickcheck;
use grok::mergesort;
use grok::quicksort;


fn main() {
	fn test_sut<F>(sut: F , input: Vec<i32>) -> bool where F : Fn(&Vec<i32>) -> Vec<i32> {
		debug!("input :     {:?}", input);
		let sorted: Vec<i32> = sut(&input);
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
	
	fn mergesort_quickcheck(xs: Vec<i32>) -> bool {
		test_sut(mergesort::sort, xs)
    }
	
	fn quicksort_quickcheck(xs: Vec<i32>) -> bool {
		test_sut(quicksort::sort, xs)
    }

	env_logger::init().unwrap();
	info!("mergesort::sort");
	quickcheck(mergesort_quickcheck as fn(Vec<i32>) -> bool);
	info!("quicksort::sort");
	quickcheck(quicksort_quickcheck as fn(Vec<i32>) -> bool);
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
//		fn test_sut<F>(sut: F, input: Vec<i32>) -> bool where F: Fn(&Vec<i32>) -> Vec<i32> {
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
//		fn mergesort_quickcheck(xs: Vec<i32>) -> bool {
//			test_sut(mergesort, xs)
//        }
//
//    	quickcheck(mergesort_quickcheck as fn(Vec<i32>) -> bool);
//	}
}
