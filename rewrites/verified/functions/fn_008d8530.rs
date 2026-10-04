// original: 0x008d8530 indexed_flag_is_two
// Looks up a per-row flag byte and reports whether its low two bits equal 2.
//
// `index` selects a row: the row's base word is read from the game's row
// table, `addend` is added, the sum is tripled to form a byte offset, and
// the byte at that offset (times 8, plus 8) from the flag area is masked
// with 3 and compared against 2. All arithmetic wraps mod 2^32.
export!(cdecl, rw_008d8530(addend: u32, index: u32) -> u32 {
    unsafe {
        let row = *(relocated(0x0130_53A8).wrapping_add(index.wrapping_mul(ROW_STRIDE))
            as *const u32);
        let key = row.wrapping_add(addend);
        let triple = key.wrapping_mul(3);
        let base = *global::<u32>(0x0103_E8D0);
        let off = triple.wrapping_mul(8).wrapping_add(8);
        let flag = *((base as *const u8).wrapping_add(off as usize));
        u32::from(flag & 3 == 2)
    }
});
