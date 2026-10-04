// original: 0x008b8140 frame_sample_publish
/// Frame sample publisher.
///
/// Advances the shared sample cursor (wrapping to 0 at the end of the table),
/// copies the three words of the selected 16-byte sample row to `out0..out2`,
/// and stores the shared active flag (nonzero test) as a byte at `out_flag`.
/// Returns `out_flag`.
export!(cdecl, rw_008b8140(out0: u32, out1: u32, out2: u32, out_flag: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x01168BB4;
        const CURSOR: u32 = 0x01160E80;
        const ROWS: u32 = 0x01168BB0;
        const ACTIVE: u32 = 0x01160E84;
        const ROW_LEN: u32 = 16;
        let count = (relocated(COUNT) as *const u16).read() as u32;
        let cursor_cell = global::<u32>(CURSOR);
        let mut cursor = cursor_cell.read();
        if (cursor as i32) >= (count as i32) {
            cursor = 0;
        }
        cursor_cell.write(cursor);
        let rows = (relocated(ROWS) as *const u32).read();
        let row = rows.wrapping_add(cursor.wrapping_mul(ROW_LEN));
        (out0 as *mut u32).write((row as *const u32).read());
        (out1 as *mut u32).write((row.wrapping_add(4) as *const u32).read());
        (out2 as *mut u32).write((row.wrapping_add(8) as *const u32).read());
        let active = (relocated(ACTIVE) as *const u32).read();
        (out_flag as *mut u8).write(if active != 0 { 1 } else { 0 });
        out_flag
    }
});
