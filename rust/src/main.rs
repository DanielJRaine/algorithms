use crate::linear::linear;
mod linear;

fn main() {
	let needle = 10;
	let haystack = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
	let result = search(needle, haystack, linear);
	println!("{:?}", result)
}

fn search<T>(needle: T, haystack: Vec<T>, strategy: fn(T, Vec<T>) -> bool) -> bool {
	strategy(needle, haystack)
}