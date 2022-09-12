pub struct Debounce<T, const N: usize> {
    prev_raw: [T; N],
    prev_debounced: T,
}

impl<T: Copy + PartialEq, const N: usize> Debounce<T, N> {
    pub fn new(initial: T) -> Self {
        Self {
            prev_raw: [initial; N],
            prev_debounced: initial,
        }
    }

    /// Handle the new state `next`.
    ///
    ///
    pub fn update(&mut self, next: T) -> Option<(T, T)> {
        let changed =
            next != self.prev_debounced && self.prev_raw.iter().all(|prev_raw| *prev_raw == next);
        self.prev_raw.copy_within(1.., 0);
        self.prev_raw[N - 1] = next;
        if changed {
            let prev = self.prev_debounced;
            self.prev_debounced = next;
            Some((prev, next))
        } else {
            None
        }
    }
}
