// original: 0x009FD220 frag_ready_flag (proposed)

/// Report whether frag processing is currently enabled.
///
/// Calls a probe callee and returns 1 unless the probe returned nonzero
/// while the global enable byte is zero, in which case returns 0. The upper
/// 24 bits of the result are the probe's own upper bits, preserved in both
/// directions.
///
/// Original: 0x009FD220 (cdecl, no arguments; boolean in `al`).
lf_checker_rt::export!(cdecl, rw_009FD220() -> u32 {
    unsafe {
        const ENABLE: u32 = 0x01980C77;
        let r = lf_checker_rt::callee_cdecl!(1, u32,);
        let flag = (lf_checker_rt::relocated(ENABLE) as *const u8).read();
        let lo = if (r as u8) == 0 || flag != 0 { 1u32 } else { 0u32 };
        (r & 0xFFFF_FF00) | lo
    }
});
