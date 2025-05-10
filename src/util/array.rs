pub trait CastExt<T, U: Default + AsMut<[T]>>: Sized + Iterator<Item = T> {
    fn cast(mut self) -> Option<U> {
        let mut out: U = U::default();
        let arr: &mut [T] = out.as_mut();
        for i in 0..arr.len() {
            match self.next() {
                None => return None, // not enough elements
                Some(v) => arr[i] = v,
            }
        }
        if self.next().is_some() {
            return None; // too many elements
        }
        Some(out)
    }
}

impl<T, U: Iterator<Item = T>, V: Default + AsMut<[T]>> CastExt<T, V> for U {}
