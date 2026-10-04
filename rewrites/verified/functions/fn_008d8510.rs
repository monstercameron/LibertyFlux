// original: 0x008d8510 dispatch_indexed_row
// Reads a row word selected by `index` from the game's row table, adds
// `addend`, and dispatches the key through the row handler. All arithmetic
// wraps mod 2^32.
export!(cdecl, rw_008d8510(addend: u32, index: u32) -> u32 {
    unsafe {
        let row = *(relocated(0x0130_53A8).wrapping_add(index.wrapping_mul(100))
            as *const u32);
        let key = row.wrapping_add(addend);
        callee_thiscall!(1, u32, relocated(0x0103_E8D0), key)
    }
});
