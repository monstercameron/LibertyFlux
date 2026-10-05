// original: 0x00c6c960 anim_lookup_word18 (proposed)

/// Look up a record by key and index the pointer table at offset 0x18.
///
/// Same shape as the 0x10 table lookup, one table slot further into the
/// record.
///
/// Original: cdecl with two stack words, one call.
lf_checker_rt::export!(cdecl, rw_00c6c960(key: u32, idx: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const TABLE_OFF: u32 = 0x18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        let tab = rd32(rec.wrapping_add(TABLE_OFF));
        rd32(tab.wrapping_add(idx.wrapping_mul(4)))
    }
});
