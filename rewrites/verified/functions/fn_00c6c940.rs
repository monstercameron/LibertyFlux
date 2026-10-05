// original: 0x00c6c940 anim_lookup_word10 (proposed)

/// Look up a record by key and index the pointer table at offset 0x10.
///
/// The callee resolves `key` to a record pointer; the table at record
/// offset 0x10 is indexed by `idx` and that entry is returned.
///
/// Original: cdecl with two stack words, one call.
lf_checker_rt::export!(cdecl, rw_00c6c940(key: u32, idx: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const TABLE_OFF: u32 = 0x10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        let tab = rd32(rec.wrapping_add(TABLE_OFF));
        rd32(tab.wrapping_add(idx.wrapping_mul(4)))
    }
});
