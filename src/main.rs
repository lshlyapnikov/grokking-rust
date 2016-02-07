fn main() {
//  let mut f = std::fs::File::open("main.rs").ok().expect("Couldn’t open foo.txt");

  let x: i32 = 100;
  let y = 200;
  println!("This is a Rust programm!");
  println!("Hello, world!");
	println!("x + y = {}", x + y);
	
	let vec0 = vec![1, 2, 3, 4];
	for x in vec0.iter() {
  	println!("vec contained {}", x);
	}
	let vec1 = vec0.iter().map(|x| x * 10);
	for x in vec1 {
  	println!("vec contained {}", x);
	}

	let plus_one = |x: i32| x + 1;
	let plus_two = |x: u32| x + 2;
	let v = plus_one(11);
	println!("v: {}", v);
	println!("plus_two: {}", plus_two(111));
	
	run();
	
	let sum:i32 = (1..6).fold(0, |z, x| z + x);
	println!("sum: {}", sum);
	
	let tmp: Vec<_> = (1..11).filter(|x| *x % 2 == 0).collect();
	for n in &tmp {
	  println!("{}", *n)
	}
	
	for n in 1..6 {
	  println!("n: {}", n);
	}
	
//	let emptyVec: Vec<i32> = ();
//	println!("emptyVec: {}", emptyVec.next())
}


fn run() {
  println!("just some output!....")
}