// original: 0x00b78b50 row_probe_or_defer (proposed)

/// Probe one table row for `arg0` unless the context is blocked.
///
/// Returns 0 at once when the context's blocked flag byte is set. Otherwise
/// reads the signed selector byte for the row (`arg0 * 80` into the byte
/// table): a negative selector accepts the row directly. A non-negative
/// selector is scaled and added to the row base and passed, with the
/// context, to the probe callee; the row is accepted when the callee's low
/// byte is non-zero. Only the low byte of the outcome is set by the
/// original; on the callee path the upper three bytes of the callee result
/// pass through, and on the early paths the caller's leftover upper bytes
/// pass through (the contract fixes entry `eax` to 0 so both sides agree).
///
/// Original: cdecl with two stack words, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b78b50(arg0: u32, ctx: u32) -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const BLOCKED: u32 = 0x219;
        const SELECTORS: u32 = 0x0167CED9;
        const ROW_STRIDE: u32 = 80;
        const PROBE_STRIDE: u32 = 0x58;
        const ROW_BASE: u32 = 0x011D9290;
        if ((ctx + BLOCKED) as *const u8).read() != 0 {
            return 0;
        }
        let sel = ((lf_checker_rt::relocated(SELECTORS) + arg0.wrapping_mul(ROW_STRIDE))
            as *const i8)
            .read();
        if sel < 0 {
            return 1;
        }
        let row = (sel as i32)
            .wrapping_mul(PROBE_STRIDE as i32)
            .wrapping_add(lf_checker_rt::relocated(ROW_BASE) as i32) as u32;
        let r: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, row, ctx);
        (r & 0xFFFF_FF00) | u32::from(r & 0xFF != 0)
    }
});
