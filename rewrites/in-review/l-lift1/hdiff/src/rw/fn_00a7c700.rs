// original: 0x00a7c700 chain_all_flagged
// Whole-chain flag check: do `this` and every +0x118 successor carry
// flag bits 2 and 3 (+0x13c & 0x0C)? Returns 1 for a fully flagged
// chain (a lone node counts), 0 at the first node that fails.
export!(thiscall, rw_s13_00a7c700(this: *const u8) -> u8 {
    unsafe {
        let mut node = this;
        loop {
            let flags = *node.add(0x13C);
            if flags & 0x04 == 0 || flags & 0x08 == 0 {
                return 0;
            }
            let next = *(node.add(0x118) as *const u32);
            if next == 0 {
                return 1;
            }
            node = next as *const u8;
        }
    }
});
