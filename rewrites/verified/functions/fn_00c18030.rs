// original: 0x00c18030 float_one_store_through_double_indirect

/// Store the float 1.0 into a slot reached through two pointers.
///
/// `this` points to a record whose dword at `+0x12c` is a pointer to a second
/// record; the rewrite stores `1.0f` (`0x3f800000`) at `+0x1450` of that
/// second record. Returns the inner pointer (left in `eax` by the original).
///
/// Original: 0x00C18030 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c18030(this: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x12c;
        const SLOT_OFF: u32 = 0x1450;
        const ONE_BITS: u32 = 0x3f80_0000;
        let inner = ((this + INNER_OFF) as *const u32).read_unaligned();
        ((inner + SLOT_OFF) as *mut u32).write_unaligned(ONE_BITS);
        inner
    }
});
