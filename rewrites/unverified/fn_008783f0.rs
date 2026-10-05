// original: 0x008783F0 conduit_op_count

/// Live-slot count of the conduit entry staged at `this + 8`.
///
/// `this` points to a conduit object. Word at `+8` points at the staged
/// entry (null means none staged); the entry's first word is a kind code.
/// The count returned is 2, 1, the staged entry's own `+0x10` word, or 0,
/// selected by the kind code minus one through the original's two-level
/// dispatch (index map, then jump table):
///
/// | kind - 1 | result |
/// |---|---|
/// | 0 | 0 |
/// | 1 | 0 |
/// | 2 | 0 |
/// | 3 | 2 |
/// | 4 | staged +0x10 word |
/// | 5 | 2 |
/// | 6 | staged +0x10 word |
/// | 7 | 1 |
/// | 8 | 1 |
/// | 9 | 1 |
/// | 10 | staged +0x10 word |
/// | 11 | 0 |
///
/// A null staged entry, or a kind outside 1..=12, gives 0. The range check
/// is UNSIGNED (`ja`): kind 0 wraps to 0xFFFF_FFFF and is rejected.
///
/// Original: 0x008783F0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008783F0(this: u32) -> u32 {
    unsafe {
        const STAGED_OFF: u32 = 0x08;
        const KIND_OFF: u32 = 0x00;
        const STAGED_COUNT_OFF: u32 = 0x10;
        const MAX_KIND_MINUS_ONE: u32 = 11;
        // kind-1 -> arm: 0 = return 2, 1 = return staged word, 2 = return 1,
        // 3 = return 0. Derived from the mapped index/jump tables.
        const ARM: [u8; 12] = [3, 3, 3, 0, 1, 0, 1, 2, 2, 2, 1, 3];
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let staged = rd32(this.wrapping_add(STAGED_OFF));
        if staged == 0 {
            return 0;
        }
        let v = rd32(staged.wrapping_add(KIND_OFF)).wrapping_sub(1);
        if v > MAX_KIND_MINUS_ONE {
            return 0;
        }
        match ARM[v as usize] {
            0 => 2,
            1 => rd32(staged.wrapping_add(STAGED_COUNT_OFF)),
            2 => 1,
            _ => 0,
        }
    }
});
