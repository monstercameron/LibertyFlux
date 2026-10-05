// original: 0x00c6c7a0 anim_lookup_word14 (proposed)

/// Look up a record by key and return the halfword at offset 0x14.
///
/// The callee resolves `key` to a record pointer; the halfword there is
/// returned zero-extended. No null check: a null answer faults on the
/// read, exactly like the original.
///
/// Original: cdecl with one stack word, one call.
lf_checker_rt::export!(cdecl, rw_00c6c7a0(key: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const WORD_OFF: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        rd16(rec.wrapping_add(WORD_OFF))
    }
});
