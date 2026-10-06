// original: 0x005B5010 refresh_marked_rows (proposed)

/// Poll every marked row of the status table and publish the results.
///
/// Walks the row table (`ROW_BASE`, `ROW_COUNT` rows of 22 bytes). A row
/// whose tag byte is 0x0e is live: the sensor callee (no arguments) is
/// called, its low byte stored into the row's value slot (+0x14), the limit
/// global is cleared when it is *unsigned*-above the full sensor answer
/// (equal or below keeps it; the comparison is `cmova`, unsigned), and the
/// value byte is published into the output table indexed by the row's signed
/// 16-bit key (+0x12). Rows with any other tag are skipped untouched. An
/// empty table returns immediately. Nothing is returned.
lf_checker_rt::export!(cdecl, rw_005B5010() -> u32 {
    unsafe {
        const ROW_BASE: u32 = 0x019D3_448;
        const ROW_COUNT: u32 = 0x019D3_44C;
        const LIMIT: u32 = 0x01284_644;
        const LIMIT_OUT: u32 = 0x01160_D6C;
        const OUTPUT: u32 = 0x01160_FE8;
        const ROW_STRIDE: u32 = 22;
        const LIVE_TAG: u8 = 0x0E;
        const VALUE_OFF: u32 = 0x14;
        const KEY_OFF: u32 = 0x12;
        const SENSOR: u32 = 1;

        let count = (lf_checker_rt::relocated(ROW_COUNT) as *const u16).read() as u32;
        if count == 0 {
            return 0;
        }
        let base = lf_checker_rt::global::<u32>(ROW_BASE).read();
        let mut i = 0u32;
        while i < count {
            let row = base.wrapping_add(i.wrapping_mul(ROW_STRIDE));
            if (row as *const u8).read() == LIVE_TAG {
                let v = lf_checker_rt::callee_cdecl!(SENSOR, u32,);
                ((row.wrapping_add(VALUE_OFF)) as *mut u8).write(v as u8);
                let lim = lf_checker_rt::global::<u32>(LIMIT).read();
                let kept = if lim > v { 0 } else { lim };
                lf_checker_rt::global::<u32>(LIMIT_OUT).write(kept);
                let key =
                    ((row.wrapping_add(KEY_OFF)) as *const i16).read_unaligned() as i32 as u32;
                let val = ((row.wrapping_add(VALUE_OFF)) as *const u8).read() as u32;
                ((lf_checker_rt::relocated(OUTPUT).wrapping_add(key.wrapping_mul(4)))
                    as *mut u32)
                    .write(val);
            }
            i += 1;
        }
        0
    }
});
