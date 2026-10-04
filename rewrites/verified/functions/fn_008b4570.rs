// original: 0x008b4570 NativeImpl_DRAW_FRONTEND_HELPER_TEXT
/// Frontend helper-text line queue.
///
/// Copies two NUL-terminated strings into the current helper-text row
/// (33-byte stride) together with the `flags` byte, and advances the row
/// cursor, saturating at 12 rows. Note the argument order, which the checker
/// proved: the second argument fills the first row bank and the first
/// argument fills the second row bank. Returns the cursor value after
/// advancing (entered cursor plus one).
export!(cdecl, rw_008b4570(row2_text: u32, row1_text: u32, flags: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x011609E8;
        const ROWS1: u32 = 0x011609F8;
        const ROWS2: u32 = 0x01160A08;
        const ROWFLAGS: u32 = 0x01160A18;
        const STRIDE: u32 = 33;
        const MAX_ROWS: u32 = 12;
        let next_cell = global::<u32>(NEXT);
        let row = next_cell.read().wrapping_mul(STRIDE);
        let mut src = row1_text;
        let mut dst = relocated(ROWS1).wrapping_add(row);
        loop {
            let b = (src as *const u8).read();
            (dst as *mut u8).write(b);
            if b == 0 {
                break;
            }
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
        }
        src = row2_text;
        dst = relocated(ROWS2).wrapping_add(row);
        loop {
            let b = (src as *const u8).read();
            (dst as *mut u8).write(b);
            if b == 0 {
                break;
            }
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
        }
        (relocated(ROWFLAGS).wrapping_add(row) as *mut u8).write(flags as u8);
        let advanced = next_cell.read().wrapping_add(1);
        if (advanced as i32) < (MAX_ROWS as i32) {
            next_cell.write(advanced);
        }
        advanced
    }
});
