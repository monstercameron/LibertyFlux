// original: 0x00A8EA70 pool_row_count_guarded (proposed)

/// Read a row's 16-bit count word when the pool is enabled.
///
/// Returns 0 when the enable byte at `this+0x73` is clear. Otherwise the
/// row table at `this+0xE4` is indexed by `a*160` and the 16-bit word at
/// row `+4` is returned zero-extended. Pure loads, no calls.
///
/// Original: thiscall, one stack word (row index), returns u32 in EAX.
lf_checker_rt::export!(thiscall, rw_00A8EA70(this: u32, row: u32) -> u32 {
    unsafe {
        const ENABLE_OFF: u32 = 0x73;
        const ROW_TABLE_OFF: u32 = 0xe4;
        const ROW_STRIDE: u32 = 160;
        const COUNT_OFF: u32 = 4;
        if ((this + ENABLE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let table = ((this + ROW_TABLE_OFF) as *const u32).read_unaligned();
        let block = (table.wrapping_add(row.wrapping_mul(ROW_STRIDE)) as *const u32)
            .read_unaligned();
        ((block + COUNT_OFF) as *const u16).read_unaligned() as u32
    }
});
