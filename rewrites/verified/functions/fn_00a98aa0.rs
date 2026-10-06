// original: 0x00a98aa0 filemem_count_ready_entries

/// Count the ready entries in every list owned by this object.
///
/// `this` points to an object whose word at `+0x10` heads a chain of outer
/// nodes linked through their first word. Each outer node points at
/// `+0x0c` to a chain of inner entries linked through their first word. An
/// inner entry counts as ready when its flag byte at `+0xf0` is non-zero
/// and its state word at `+0xf4` is zero. Returns the count (32-bit
/// unsigned; the original walks both chains to their null terminators).
///
/// Original: 0x00A98AA0 (thiscall, no stack arguments, leaf).
lf_checker_rt::export!(thiscall, rw_00a98aa0(this: u32) -> u32 {
    unsafe {
        /// Head of the outer-node chain, from the object base.
        const HEAD_OFF: u32 = 0x10;
        /// Next-outer link, from an outer node.
        const OUTER_NEXT: u32 = 0x00;
        /// Head of the inner-entry chain, from an outer node.
        const OUTER_INNER: u32 = 0x0c;
        /// Next-entry link, from an inner entry.
        const INNER_NEXT: u32 = 0x00;
        /// Ready-flag byte, from an inner entry.
        const INNER_FLAG: u32 = 0xf0;
        /// State word (must be zero), from an inner entry.
        const INNER_STATE: u32 = 0xf4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut count: u32 = 0;
        let mut outer = rd32(this.wrapping_add(HEAD_OFF));
        while outer != 0 {
            let mut inner = rd32(outer.wrapping_add(OUTER_INNER));
            while inner != 0 {
                let flag = ((inner.wrapping_add(INNER_FLAG)) as *const u8).read();
                if flag != 0 && rd32(inner.wrapping_add(INNER_STATE)) == 0 {
                    count = count.wrapping_add(1);
                }
                inner = rd32(inner.wrapping_add(INNER_NEXT));
            }
            outer = rd32(outer.wrapping_add(OUTER_NEXT));
        }
        count
    }
});
