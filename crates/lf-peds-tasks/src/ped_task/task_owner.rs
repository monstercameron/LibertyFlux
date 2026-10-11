//! The task's owner object slot: one pointer, replaced by a new object.
//!
//! The task keeps a pointer to the object that owns it. Replacing the
//! pointer releases the previous object and deallocates it, but only when
//! the previous pointer is nonnull and differs from the replacement. The
//! release and the deallocation are helper calls, and travel as
//! [`OwnerCallees`].

/// The owner pointer, owned.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TaskOwner {
    /// The owner object pointer.
    pub object: u32,
}

/// The helper calls of an owner replacement. Each receives the previous
/// object.
pub trait OwnerCallees {
    /// The release helper, a method of the previous object.
    fn release(&mut self, previous: u32);
    /// The deallocator for the previous object.
    fn deallocate(&mut self, previous: u32);
}

impl TaskOwner {
    /// Stores `new_object` as the owner. A nonnull previous owner that differs
    /// from the new one is released and then deallocated before the store.
    /// Other cases only store.
    pub fn replace_owner<C: OwnerCallees>(&mut self, new_object: u32, callees: &mut C) {
        let previous = self.object;
        if previous != 0 && previous != new_object {
            callees.release(previous);
            callees.deallocate(previous);
        }
        self.object = new_object;
    }
}
