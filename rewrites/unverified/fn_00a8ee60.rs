// original: 0x00A8EE60 pool_indexed_cell (proposed)

/// Fetch a cell through the index helper with no bounds check.
///
/// The index helper runs with (`a`, `b`); the table at `this+0x80` is read
/// at byte offset `96 * r + 0x50`.
///
/// Original: thiscall, two stack words, returns u32 in EAX. One outgoing
/// call (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00A8EE60(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x80;
        const STRIDE: u32 = 96;
        const CELL_OFF: u32 = 0x50;
        const HELPER: u32 = 1;
        let r: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this, a, b);
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        (table.wrapping_add(r.wrapping_mul(STRIDE)).wrapping_add(CELL_OFF) as *const u32)
            .read_unaligned()
    }
});
