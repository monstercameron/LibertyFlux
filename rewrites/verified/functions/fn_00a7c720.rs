// original: 0x00a7c720 chain_set_flags
// Set flag bits 2 and 3 (+0x13c |= 0x0C) on `this` and every +0x118
// successor. Returns nothing.
export!(thiscall, rw_s13_00a7c720(this: *mut u8) -> () {
    unsafe {
        let mut node = this;
        loop {
            *node.add(0x13C) |= 0x0C;
            let next = *(node.add(0x118) as *const u32);
            if next == 0 {
                return;
            }
            node = next as *mut u8;
        }
    }
});
