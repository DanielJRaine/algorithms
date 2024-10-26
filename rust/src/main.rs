#![feature(isqrt, iter_advance_by)]
mod challenges {
    pub mod two_crystal_balls;
}

mod search {
    pub mod binary;
    pub mod linear;
}

mod sort {
    pub mod bubble;
}

fn main() {
    // let needle = 11;
    // let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let result = search(needle, haystack, binary::<usize>);
    // println!("{:?}", result)

	// let b = two_crystal_balls([false,false,false,false,false,false,false,false,false,true]);
	// println!("{b}");


}

fn search(needle: usize, haystack: [usize; 10], strategy: fn(usize, [usize; 10]) -> bool) -> bool {
    strategy(needle, haystack)
}

fn sort(collection: [usize; 10], strategy: fn([usize; 10]) -> bool) -> bool {
    strategy(collection)
}
