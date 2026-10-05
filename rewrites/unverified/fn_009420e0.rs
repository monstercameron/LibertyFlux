// original: 0x009420e0 streaming_slot_clear (proposed)

/// Clear the indexed slot pointer in the object's slot table.
///
/// Reads the index at `this + 0x2040` and stores zero to the table entry at
/// `this + 0x10 + index * 4`. Returns the index in `eax`.
///
/// Original: 0x009420e0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009420e0(this: u32) -> u32 {
    unsafe {
        const INDEX: u32 = 0x2040;
        const TABLE: u32 = 0x10;
        let index = ((this + INDEX) as *const u32).read_unaligned();
        ((this + TABLE + index.wrapping_mul(4)) as *mut u32).write_unaligned(0);
        index
    }
});
