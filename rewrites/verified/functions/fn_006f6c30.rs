// original: 0x006f6c30 resolve_chain_tail
/// Resolve the tail of a node chain into an output record.
///
/// Walks the `+0x10` links from the head at `+0` to the last node, then
/// writes a record to `out`: the tail's word at `+0x18` (or zero), the
/// tail pointer (or null), and `this`. Returns `out`.
rt::export!(thiscall, rw_006f6c30(this: *const u8, out: *mut u8) -> u32 {
    unsafe {
        let first = *this.cast::<u32>();
        let mut tail = first;
        if first != 0 {
            let mut next = *((first as *const u8).add(0x10).cast::<u32>());
            while next != 0 {
                tail = next;
                next = *((tail as *const u8).add(0x10).cast::<u32>());
            }
        } else {
            tail = 0;
        }
        *out.cast::<u16>() = 0;
        *out.add(4).cast::<u32>() = 0;
        *out.add(8).cast::<u32>() = this as u32;
        if tail != 0 {
            *out.cast::<u16>() = *((tail as *const u8).add(0x18).cast::<u16>());
            *out.add(4).cast::<u32>() = tail;
        } else {
            *out.add(4).cast::<u32>() = 0;
        }
        out as u32
    }
});
