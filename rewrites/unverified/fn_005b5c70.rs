// original: 0x005B5C70 invalidate_rows_by_id (proposed)

/// Invalidate every row carrying the given id.
///
/// Walks the table (`this`: row base at +0, 16-bit count at +4; rows are 16
/// bytes). A row whose id word matches the argument is invalidated: its
/// state word (+4) is cleared to 0 unless it already holds 5 (5 is kept as
/// is, a quirk, not a second clear), and its two payload words (+8, +0xc)
/// are set to -1. Rows with other ids are untouched. An empty table returns
/// immediately. Nothing is returned. Thiscall: table in ECX, id on the stack.
lf_checker_rt::export!(thiscall, rw_005B5C70(this: u32, id: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const ROW_STRIDE: u32 = 16;
        const ID_OFF: u32 = 0;
        const STATE_OFF: u32 = 4;
        const PAYLOAD_A: u32 = 8;
        const PAYLOAD_B: u32 = 0x0C;
        const KEEP_STATE: u32 = 5;

        let base = (this.wrapping_add(ROWS) as *const u32).read();
        let count = ((this.wrapping_add(COUNT)) as *const u16).read() as u32;
        let mut i = 0u32;
        while i < count {
            let row = base.wrapping_add(i.wrapping_mul(ROW_STRIDE));
            if ((row.wrapping_add(ID_OFF)) as *const u32).read() == id {
                let state = (row.wrapping_add(STATE_OFF)) as *mut u32;
                if state.read() != KEEP_STATE {
                    state.write(0);
                }
                ((row.wrapping_add(PAYLOAD_A)) as *mut u32).write(0xFFFF_FFFF);
                ((row.wrapping_add(PAYLOAD_B)) as *mut u32).write(0xFFFF_FFFF);
            }
            i += 1;
        }
        0
    }
});
