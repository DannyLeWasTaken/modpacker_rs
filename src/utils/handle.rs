use std::marker::PhantomData;


pub struct Handle<T> {
    index: i64,
    count: i64,
    resource_type: PhantomData<T>,
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        other.count == self.count &&
            other.index == self.index
    }
}

impl<T> Default for Handle<T> {
    // Default to invalid for now
    fn default() -> Self {
        Handle {
            index: -1,
            count: -1,
            resource_type: PhantomData,
        }
    }
}

impl<T> Handle<T> {
    fn new() -> Self {
        Default::default()
    }

    fn valid(&self) -> bool {
        self.index > -1 && self.count > -1
    }
}