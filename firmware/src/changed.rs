use core::ops::{Deref, DerefMut};

pub struct Changed<T> {
    changed: bool,
    value: T,
}

impl<T> Changed<T> {
    pub fn new(value: T) -> Self {
        Self {
            changed: false,
            value,
        }
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }

    pub fn take(&mut self) -> Option<&T> {
        if self.changed {
            self.changed = false;
            Some(&self.value)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.changed = true;
        &mut self.value
    }
}

impl<T> Deref for Changed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Changed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
