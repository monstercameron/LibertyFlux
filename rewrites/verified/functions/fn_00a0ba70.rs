// original: 0x00a0ba70 pool_row_pairs_zero (proposed)
/// Zero the two pointer slots of each of 16 rows in a pool table.
///
/// Starting at `this + 0x84`, 16 times: zero the dwords at cursor-4 and
/// cursor, then advance the cursor by 0x30. Covers `this + 0x80` up to
/// `this + 0x380`. Returns the end cursor (`this + 0x384`). Thiscall.
/// (The inventory lists this function as 33 bytes; the loop-close jump and
/// the return sit just past that, making 36.)
lf_checker_rt::export!(thiscall, rw_00a0ba70(this: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x10;
        const STRIDE: u32 = 0x30;
        let mut cur = this + 0x84;
        let mut n = ROWS;
        while n != 0 {
            ((cur - 4) as *mut u32).write_unaligned(0);
            (cur as *mut u32).write_unaligned(0);
            cur += STRIDE;
            n -= 1;
        }
        cur
    }
});
