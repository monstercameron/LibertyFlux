// original: 0x00E672B0 init_six_and_zero_flag

/// Initialise six fixed objects, then clear a flag global.
///
/// Calls the callee once per entry of `OBJS` with the entry in ECX, writes
/// zero to the flag global `FLAG`, and returns the last call's answer. Object
/// addresses are loader-relocated in the original and derived from the
/// relocated image base here.
///
/// Original: 0x00E672B0 (cdecl, no arguments, six outgoing calls, returns last result).
lf_checker_rt::export!(cdecl, rw_00e672b0() -> u32 {
    unsafe {
        const OBJS: [u32; 6] = [0x012DDF60, 0x012DDFB0, 0x012DE000, 0x012DE050, 0x012DE0A0, 0x012DE0F0];
        const FLAG: u32 = 0x012DE140;
        let mut last: u32 = 0;
        for o in OBJS {
            last = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(o));
        }
        (lf_checker_rt::global::<u32>(FLAG)).write_unaligned(0);
        last
    }
});
