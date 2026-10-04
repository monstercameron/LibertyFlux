// original: 0x00a7cac0 splice_node
// Splice `new_node` into the doubly linked chain ahead of `this`:
// the predecessor (when present) is re-hung onto `new_node`, and
// `new_node` takes over `this`'s successor with back-links both ways.
// Returns `new_node`.
export!(thiscall, rw_s13_00a7cac0(this: *mut u8, new_node: u32) -> u32 {
    unsafe {
        let prev = *(this.add(0x11C) as *const u32);
        let next = *(this.add(0x118) as *const u32);
        if prev != 0 {
            *((prev + 0x120) as *mut u32) = new_node;
        }
        *(this.add(0x11C) as *mut u32) = new_node;
        *((new_node + 0x118) as *mut u32) = next;
        *((new_node + 0x120) as *mut u32) = this as u32;
        *((new_node + 0x11C) as *mut u32) = prev;
        new_node
    }
});
