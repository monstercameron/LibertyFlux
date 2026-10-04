// original: 0x00cb7fa0 aimed_sweep_test
/// Run an aimed sweep and test whether its count stayed low (1 call).
///
/// Zeroes the sweep-count global at file address `0x0171BBFC` and stores
/// `a2` in the sweep-argument global at `0x0171BC00` (stdcall, three stack
/// arguments). Builds the 3-word probe `[a0.x, a1, a0.z]` on its frame
/// (the middle probe word the original loads from `[a0 + 4]` is
/// overwritten by `a1` before the call, so only `a0.x`/`a0.z` matter) and
/// calls the five-argument sweep with (probe, callback `0x00CBB190`, 0, 8,
/// `0xD`). Returns whether the count global is still signed-`<= a2`; only
/// the low byte is compared (the original's eax still holds a frame
/// address above it). The callee is intercepted by the checker; the probe
/// address is skipped and its three words are snapped.
lf_checker_rt::export!(stdcall, rw_00cb7fa0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        use lf_checker_rt::global;
        /// Globals written before the sweep (file VAs).
        const COUNT_G: u32 = 0x0171BBFC;
        const ARG_G: u32 = 0x0171BC00;
        /// Sweep callback pushed raw (no reloc fixup on the push).
        const CALLBACK: u32 = 0x00CBB190;
        /// Callee id of the sweep.
        const SWEEP: u32 = 1;
        *global::<u32>(COUNT_G) = 0;
        *global::<u32>(ARG_G) = a2;
        let x = (a0 as *const u32).read_unaligned();
        let z = ((a0 + 8) as *const u32).read_unaligned();
        let probe = [x, a1, z];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SWEEP, u32, probe.as_ptr() as u32, CALLBACK, 0, 8, 0xD);
        let g = *global::<u32>(COUNT_G);
        u32::from((g as i32) <= (a2 as i32))
    }
});
