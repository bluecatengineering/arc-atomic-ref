# arc-atomic-ref

an `AtomicRef` is a smart pointer type that can be shared with many
different threads of execution, while at the same time can be swapped out
atomically with new data. In this way, it's similar to a lock-free
`RwLock` or `Mutex` when you can replace the contained data rather than
modify it.

```rust
use arc_atomic_ref::AtomicRef;
use std::sync::Arc;
let ptr = AtomicRef::new(1);
// share ptr with many threads with `clone`
// change its contained value, requires a new `Arc`
ptr.swap(Arc::new(2));
// all threads should see the change, use `.load()` to get the value
assert_eq!(**ptr.load(), 2);
```
