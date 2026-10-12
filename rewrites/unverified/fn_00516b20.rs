// original: 0x00516B20 rage::rlConcreteLeaderboard::get_row
///
/// Return a pointer to row `index` of the leaderboard's row array, or null.
///
/// `this` points to the board: the row count lives at +8 and the row array
/// base at +0x0C. Rows are 0x138 bytes apart. The bounds checkrw_00516b20: an
/// index at or past the count yields null, otherwise base + index * 0x138
/// with 32-bit wrapping.
///
/// thiscall: the board pointer arrives in ECX, the index on the stack.
lf_checker_rt::export!(thiscall,  (unsigned, as the original's `jae`)(this: u32, index: u32) -> u32 {
    const COUNT_AT: u32 = 8;
    const ROWS_AT: u32 = 0x0C;
    const ROW_STRIDE: u32 = 0x138;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let count = read_u32(this.wrapping_add(COUNT_AT));
        if index >= count {
            return 0;
        }
        let rows = read_u32(this.wrapping_add(ROWS_AT));
        rows.wrapping_add(index.wrapping_mul(ROW_STRIDE))
    }
});
