use std::sync::atomic::{AtomicUsize, Ordering};

pub trait OperationData: Clone + Send + Sync + 'static {
    fn get(&self) -> usize;
    fn increment(&self);
    fn decrement(&self);
}

pub struct SharedData {
    pub counter: AtomicUsize,
}

impl SharedData {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }
}

impl Clone for SharedData {
    fn clone(&self) -> Self {
        Self {
            counter: AtomicUsize::new(self.counter.load(Ordering::Relaxed)),
        }
    }
}

impl OperationData for SharedData {
    fn get(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }
    fn increment(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }
    fn decrement(&self) {
        self.counter.fetch_sub(1, Ordering::Relaxed);
    }
}
