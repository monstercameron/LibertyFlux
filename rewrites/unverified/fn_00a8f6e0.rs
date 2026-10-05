// original: 0x00A8F6E0 pool_triple_query_cell_test (proposed)

/// Test whether the cell named by a triple query is empty.
///
/// Three frame slots seeded to `0x3F` are filled by the query helper (with
/// `n`); 1 is returned unless the first slot kept the sentinel while the
/// second changed. In that case the table at `this+0x80`, indexed by
/// `third + 24 * second`, has its word `+0x40` tested for zero (1 when
/// zero). Only the low byte of the return is set on one path.
///
/// Original: thiscall, one stack word, low byte in AL. One outgoing call
/// (thiscall, four stack words, three frame out-slots).
lf_checker_rt::export!(thiscall, rw_00A8F6E0(this: u32, n: u32) -> u32 {
    unsafe {
        const SENTINEL: u32 = 0x3f;
        const TABLE_OFF: u32 = 0x80;
        const STRIDE: u32 = 24;
        const CELL_OFF: u32 = 0x40;
        const QUERY: u32 = 1;
        let mut first = SENTINEL;
        let mut second = SENTINEL;
        let mut third = SENTINEL;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            QUERY,
            u32,
            this,
            n,
            &mut first as *mut u32 as u32,
            &mut second as *mut u32 as u32,
            &mut third as *mut u32 as u32
        );
        if first != SENTINEL {
            return 1;
        }
        if second == SENTINEL {
            return 1;
        }
        let t = third.wrapping_add(second.wrapping_mul(STRIDE));
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let v = (table.wrapping_add(t.wrapping_mul(4)).wrapping_add(CELL_OFF) as *const u32)
            .read_unaligned();
        (v == 0) as u32
    }
});
