// original: 0x00a7c9c0 last_link_in_chain
// Follow +0x124 once, then walk +0x11c links to the end. Returns the
// last node reached, or null when the +0x124 anchor is null.
export!(thiscall, rw_s13_00a7c9c0(this: *const u8) -> u32 {
    unsafe {
        let mut node = *(this.add(0x124) as *const u32);
        if node == 0 {
            return 0;
        }
        loop {
            let prev = *((node + 0x11C) as *const u32);
            if prev == 0 {
                return node;
            }
            node = prev;
        }
    }
});
