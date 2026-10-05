// original: 0x008CA810 stream_fill_entries (proposed)

/// Fills `count` streaming entries from `array` (an array of 0x10-byte
/// records): does nothing when `count` is zero or negative. Each iteration
/// re-reads the device object from the `DEVICE` global, refreshes it through
/// the refresh callee (callee 0), resolves one entry id through the resolve
/// callee (callee 1, cdecl with the id word just below the current record:
/// at `-8` when the device's flag byte at `+DEV_FLAG` is set, at `-12`
/// otherwise), refreshes the device again, and, when the flag byte is set
/// and `store` is non-zero, writes the resolved id into the record.
///
/// Three stack arguments (cdecl): `store`, `count`, `array`. No result.
lf_checker_rt::export!(cdecl, rw_008CA810(store: u32, count: u32, array: u32) -> u32 {
    unsafe {
        /// Refresh callee id.
        const REFRESH: u32 = 0;
        /// Resolve callee id.
        const RESOLVE: u32 = 1;
        /// Global holding the device object pointer.
        const DEVICE: u32 = 0x1BB5624;
        /// Offset of the flag byte in the device object.
        const DEV_FLAG: u32 = 0x169;
        /// Record stride in bytes.
        const STRIDE: u32 = 0x10;
        /// Record header skipped before the first id slot.
        const HEADER: u32 = 0x0C;
        if (count as i32) <= 0 {
            return 0;
        }
        let mut rec = array.wrapping_add(HEADER);
        let mut left = count;
        loop {
            let device = (lf_checker_rt::relocated(DEVICE) as *const u32).read();
            let _f: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, device);
            let id_off = if ((device + DEV_FLAG) as *const u8).read() != 0 { 8u32 } else { 12u32 };
            let want = ((rec - id_off) as *const u32).read_unaligned();
            let got: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, want);
            let device = (lf_checker_rt::relocated(DEVICE) as *const u32).read();
            let _f: u32 = lf_checker_rt::callee_thiscall!(REFRESH, u32, device);
            if ((device + DEV_FLAG) as *const u8).read() != 0 && store & 0xFF != 0 {
                (rec as *mut u32).write_unaligned(got);
            }
            rec = rec.wrapping_add(STRIDE);
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
        }
        0
    }
});
