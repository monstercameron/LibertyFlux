// original: 0x00A8EDD0 pool_indexed_cell_bounded (proposed)

/// Fetch a cell through the index helper when it is below the row limit.
///
/// The index helper runs with (`a`, `b`); its signed result at or above the
/// 16-bit limit at `this+0x84` returns 0, otherwise the table at `this+0x80`
/// is read at `base + 24 * r`, word `+0x40`.
///
/// Original: thiscall, three stack words, returns u32 in EAX. One outgoing
/// call (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00A8EDD0(this: u32, a: u32, b: u32, base: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x80;
        const LIMIT_OFF: u32 = 0x84;
        const STRIDE: u32 = 24;
        const CELL_OFF: u32 = 0x40;
        const HELPER: u32 = 1;
        let r: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this, a, b);
        let limit = ((this + LIMIT_OFF) as *const u16).read_unaligned() as u32;
        if (r as i32) >= limit as i32 {
            return 0;
        }
        let t = base.wrapping_add((r as i32 as u32).wrapping_mul(STRIDE));
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        (table.wrapping_add(t.wrapping_mul(4)).wrapping_add(CELL_OFF) as *const u32)
            .read_unaligned()
    }
});
