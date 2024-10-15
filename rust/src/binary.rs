pub fn binary<Usize: PartialEq + PartialOrd<usize>>(needle: usize, haystack: [usize; 10]) -> bool {
    let mut lo: usize = 0;
    let mut hi = haystack.len() - 1;

    let lo_val = &haystack[lo];
    if &needle < lo_val {
        return false;
    }

    loop {
        if lo == hi + 1 {
            return false;
        }

        let mid = lo + (hi - lo) / 2;
        let mid_val = &haystack[mid];

        if mid_val == &needle {
            return true;
        } else if mid_val > &needle {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::{fixture, rstest};
    use crate::search;

    // // #[fixture]
    // // fn NEEDLE() -> u32 { 10 };
    // const NEEDLE: usize = 10;
    // const HAYSTACK: [usize; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //
    // #[test]
    // fn it_finds() {
    // 	let result = search(NEEDLE, HAYSTACK, binary);
    // 	assert!(result)
    // }
    //
    // #[test]
    // fn it_fails() {
    // 	let result = search(0, HAYSTACK, binary);
    // 	assert_ne!(result, true)
    // }
}
