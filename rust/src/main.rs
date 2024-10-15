#![feature(isqrt, iter_advance_by)]

// use crate::linear::linear;
use crate::binary::binary;
use crate::two_crystal_balls::two_crystal_balls;

mod binary;
mod linear;
mod two_crystal_balls;

fn main() {
    // let needle = 11;
    // let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let result = search(needle, haystack, binary::<usize>);
    // println!("{:?}", result)
	
	let b = two_crystal_balls([false,false,false,false,false,false,false,false,false,true]);
	println!("{b}");
}

fn search(needle: usize, haystack: [usize; 10], strategy: fn(usize, [usize; 10]) -> bool) -> bool {
    strategy(needle, haystack)
}
