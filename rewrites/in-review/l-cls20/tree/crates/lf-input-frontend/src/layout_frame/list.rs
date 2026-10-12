//! The pointer-list release of the layout frame.
//!
//! The frame holds a pointer list: a base address and an unsigned 16-bit
//! count. The release walks the list from its base, and for each non-null
//! entry it runs the entry's own teardown and then frees the entry. Once the
//! walk finishes it reports the list field and the walked range to the list's
//! owner, and answers zero.
//!
//! The list entries and the teardown and free routines are not owned by the
//! frame, so the method takes a [`ListHost`]. The host answers each entry
//! and receives each call in the order the walk makes them.

use super::frame::{UILayoutFrame, field};

/// Size of one list entry: a pointer, four bytes in the 32-bit form.
const ENTRY_BYTES: u32 = 4;

/// The calls the release makes outside the frame's bytes.
pub trait ListHost {
    /// The entry at `index` of the list that starts at `list`. The index is
    /// below the count the frame holds.
    fn list_entry(&mut self, list: u32, index: usize) -> u32;

    /// The entry's own teardown, run on each non-null entry before it is freed.
    fn release_entry(&mut self, entry: u32);

    /// Frees one non-null entry, after its teardown.
    fn free_entry(&mut self, entry: u32);

    /// Reports the list field of the frame, and the range the walk covered,
    /// to the list's owner. The frame is passed so the owner can address its
    /// list field; the walked range is `base` up to `end`.
    fn list_owner_released(&mut self, frame: &mut UILayoutFrame, base: u32, end: u32);
}

impl UILayoutFrame {
    /// Walks the frame's pointer list, releasing and freeing each non-null
    /// entry in order, then reports the list field and the walked range to the
    /// owner. The walk reads its base and count once, before the first entry;
    /// an empty list makes no entry call and still reports. Answers zero.
    pub fn release_pointer_list_entries<H: ListHost>(&mut self, host: &mut H) -> u32 {
        let base = self.word_at(field::LIST_BASE);
        let count = self.half_at(field::LIST_COUNT);
        let end = base.wrapping_add(u32::from(count).wrapping_mul(ENTRY_BYTES));
        if base != end {
            for index in 0..usize::from(count) {
                let entry = host.list_entry(base, index);
                if entry != 0 {
                    host.release_entry(entry);
                    host.free_entry(entry);
                }
            }
        }
        host.list_owner_released(self, base, end);
        0
    }
}
