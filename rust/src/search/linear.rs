pub fn linear<T: std::cmp::PartialEq>(needle: T, haystack: [T; 10]) -> bool {
    for n in haystack {
        if n == needle {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::{fixture, rstest};
    use crate::search;

    // #[fixture]
    // fn NEEDLE() -> usize { 10 };
    const NEEDLE: usize = 10;
    const HAYSTACK: [usize; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    #[test]
    fn it_finds() {
        let result = search(NEEDLE, HAYSTACK, linear);
        assert!(result)
    }

    #[test]
    fn it_fails() {
        let result = search(0, HAYSTACK, linear);
        assert_ne!(result, true)
    }
}
