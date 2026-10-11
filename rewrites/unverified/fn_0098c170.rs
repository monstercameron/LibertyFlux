// original: 0x0098C170 audPoliceScanner::is_scanning

/// Return whether the scanner state word at `this + 0xB0C` is exactly 2.
///
/// This is a `thiscall` leaf with no stack arguments. The state is compared as
/// an unsigned 32-bit word; the result is a byte boolean in AL.
lf_checker_rt::export!(thiscall, rw_aud_police_scanner_is_scanning(this: u32) -> u32 {
    unsafe {
        const SCAN_STATE: u32 = 0xB0C;
        let state = (this.wrapping_add(SCAN_STATE) as *const u32).read_unaligned();
        u32::from(state == 2)
    }
});
