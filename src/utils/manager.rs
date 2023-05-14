use crate::utils::handle::Handle;

struct Manager<T> {
    handles: Vec<Handle<T>>,
    storage: Vec<T>,

    counter: u64,
}

impl<T> Manager<T> {
    fn new() -> Self {
        Manager {
            handles: Vec::new(),
            storage: Vec::new(),
            counter: 0,

        }
    }
}