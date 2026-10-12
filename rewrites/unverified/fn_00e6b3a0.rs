// original: 0x00E6B3A0 zone_volume_records_initialize (proposed)

/// Initialise every record of a fixed table by calling one helper per row.
///
/// Calls the record helper (thiscall, row pointer in ECX, no stack
/// arguments) for each of the 1550 rows starting at
/// `0x016ed3d8`, stepping `0x1c` bytes per row. The value
/// left in EAX is the last call's answer. Preserves ESI and EDI.
///
/// Original: 0x00E6B3A0 (cdecl, no arguments; counted-down loop).
lf_checker_rt::export!(cdecl, rw_00E6B3A0() -> u32 {
    unsafe {
        const FIRST: u32 = 0x016ed3d8;
        const STRIDE: u32 = 0x1c;
        const COUNT: u32 = 1550;
        const ROW_CALLEE: u32 = 1;
        let mut row = lf_checker_rt::relocated(FIRST);
        let mut last: u32 = 0;
        let mut i: u32 = 0;
        while i < COUNT {
            last = lf_checker_rt::callee_thiscall!(ROW_CALLEE, u32, row);
            row = row.wrapping_add(STRIDE);
            i += 1;
        }
        last
    }
});
