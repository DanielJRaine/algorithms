pub fn linear<T: std::cmp::PartialEq>(needle: T, haystack: Vec<T>) -> bool {
	for n in haystack {
		if n == needle { return true }
	}
	return false
}
