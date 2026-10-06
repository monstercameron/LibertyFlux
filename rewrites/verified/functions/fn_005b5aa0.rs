// original: 0x005B5AA0 publish_tagged_rows (proposed)

/// Set and publish the value of every tagged row in all 75 row tables.
///
/// Each of the 75 table slots holds a row base and a 16-bit row count, with
/// slots 24 bytes apart. Every row is 22 bytes; a row whose tag byte is
/// 0x0c gets its value byte (+0x14) and marker (+0x15) set from the mode
/// global — (4, 0x15) when the mode is *signed*-above 1, (2, 0x14)
/// otherwise — and the value byte is then published into the output table
/// indexed by the row's signed 16-bit key (+0x12). Rows with any other tag
/// are skipped. Empty tables cost only the count check. Nothing is returned.
lf_checker_rt::export!(cdecl, rw_005B5AA0() -> u32 {
    unsafe {
        const MODE: u32 = 0x01030_08C;
        const TABLE: u32 = 0x019D3_3A0;
        const TABLES: u32 = 75;
        const SLOT_STRIDE: u32 = 24;
        const COUNT_OFF: u32 = 4;
        const ROW_STRIDE: u32 = 22;
        const LIVE_TAG: u8 = 0x0C;
        const VALUE_OFF: u32 = 0x14;
        const MARKER_OFF: u32 = 0x15;
        const KEY_OFF: u32 = 0x12;
        const OUTPUT: u32 = 0x01160_FE8;

        let mode = lf_checker_rt::global::<i32>(MODE).read();
        let (value, marker) = if mode > 1 { (4u8, 0x15u8) } else { (2u8, 0x14u8) };
        let table = lf_checker_rt::relocated(TABLE);
        for t in 0..TABLES {
            let slot = table.wrapping_add(t.wrapping_mul(SLOT_STRIDE));
            let base = (slot as *const u32).read();
            let count = ((slot.wrapping_add(COUNT_OFF)) as *const u16).read() as u32;
            let mut off = 0u32;
            let mut row = 0u32;
            while row < count {
                let r = base.wrapping_add(off);
                if (r as *const u8).read() == LIVE_TAG {
                    ((r.wrapping_add(VALUE_OFF)) as *mut u8).write(value);
                    ((r.wrapping_add(MARKER_OFF)) as *mut u8).write(marker);
                    let key = ((r.wrapping_add(KEY_OFF)) as *const i16).read_unaligned() as i32
                        as u32;
                    let v = ((r.wrapping_add(VALUE_OFF)) as *const u8).read() as u32;
                    ((lf_checker_rt::relocated(OUTPUT).wrapping_add(key.wrapping_mul(4)))
                        as *mut u32)
                        .write(v);
                }
                off += ROW_STRIDE;
                row += 1;
            }
        }
        0
    }
});
