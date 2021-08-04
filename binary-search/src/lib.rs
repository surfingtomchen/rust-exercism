pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    match array.len() {
        0 => None,
        1 => (array[0] == key).then(|| 0),
        n => {
            let mid = n / 2;
            if key == array[mid] {
                return Some(mid);
            }
            let (left, right) = array.split_at(mid);
            if key < array[mid] {
                find(left, key)
            } else {
                find(right, key).map(|x| x + mid)
            }
        }
    }
}
