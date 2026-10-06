// original: 0x005B5D30 set_rows_by_tag (proposed)

/// Fill the payload of every row tagged 1.
///
/// Walks the table (`this`: row base at +0, 16-bit count at +4; rows are 16
/// bytes). A row whose tag word is 1 gets its three payload words set from
/// the three stack arguments in order (+4, +8, +0xc). Rows with any other
/// tag are untouched. An empty table returns immediately. Nothing is
/// returned. Thiscall: table in ECX, the three values on the stack.
lf_checker_rt::export!(thiscall, rw_005B5D30(this: u32, v0: u32, v1: u32, v2: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const ROW_STRIDE: u32 = 16;
        const TAG_OFF: u32 = 0;
        const LIVE_TAG: u32 = 1;

        let base = (this.wrapping_add(ROWS) as *const u32).read();
        let count = ((this.wrapping_add(COUNT)) as *const u16).read() as u32;
        let mut i = 0u32;
        while i < count {
            let row = base.wrapping_add(i.wrapping_mul(ROW_STRIDE));
            if ((row.wrapping_add(TAG_OFF)) as *const u32).read() == LIVE_TAG {
                ((row.wrapping_add(4)) as *mut u32).write(v0);
                ((row.wrapping_add(8)) as *mut u32).write(v1);
                ((row.wrapping_add(12)) as *mut u32).write(v2);
            }
            i += 1;
        }
        0
    }
});
