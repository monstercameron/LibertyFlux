// original: 0x00b57260 set_or_clear_matched_mask
// rs05f10: set or clear mask bits on matching list entries.
//
// Walks the list at `this+0x1A28` with the same filter as the notify pair;
// when `set` is nonzero ORs `mask` into the entry word at `+0x8`, otherwise
// clears those bits (skipping the store when no bit is shared, as the
// original does).
export!(thiscall, rw_b57260(this: *const u8, wanted: u32, mask: u32, set: u32) -> () {
    unsafe {
        let mut node = *(this.add(0x1A28) as *const *const u8);
        while !node.is_null() {
            let next = *(node.add(0x8C) as *const *const u8);
            let body = node.add(4);
            if *(node.add(0x48) as *const u16) == 1
                && *(body.add(0x40) as *const u32) != 0
                && *(body.add(8) as *const u32) == wanted
            {
                let slot = body.add(4) as *mut u32;
                if set as u8 != 0 {
                    *slot |= mask;
                } else {
                    let v = *slot;
                    if v & mask != 0 {
                        *slot = v & !mask;
                    }
                }
            }
            node = next;
        }
    }
});
