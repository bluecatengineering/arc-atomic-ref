//! # arc-atomic-ref
//!
//! an `AtomicRef` is a smart pointer type that can be shared with many
//! different threads of execution, while at the same time can be swapped out
//! atomically with new data. In this way, it's similar to a lock-free
//! `RwLock` or `Mutex` when you can replace the contained data rather than
//! modify it.
//!
//! ```rust
//! use atomic_ref::AtomicRef;
//! use std::sync::Arc;
//! let ptr = AtomicRef::new(1);
//! // share ptr with many threads with `clone`
//! // change its contained value, requires a new `Arc`
//! ptr.swap(Arc::new(2));
//! // all threads should see the change, use `.load()` to get the value
//! assert_eq!(**ptr.load(), 2);
//! ```
use std::{ops::Deref, sync::Arc};

use arc_swap::ArcSwap;

/// Stores data in a location accessible to multiple threads but also atomically
/// swappable
#[derive(Debug, Default)]
pub struct AtomicRef<T>(Arc<ArcSwap<T>>);

impl<T> AtomicRef<T> {
    /// Create new AtomicRef from some `T`
    pub fn new(store: T) -> Self {
        Self(Arc::new(ArcSwap::new(Arc::new(store))))
    }
}

impl<T> From<Arc<T>> for AtomicRef<T> {
    fn from(store: Arc<T>) -> Self {
        Self(Arc::new(ArcSwap::new(store)))
    }
}

impl<T> Deref for AtomicRef<T> {
    type Target = ArcSwap<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for AtomicRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
