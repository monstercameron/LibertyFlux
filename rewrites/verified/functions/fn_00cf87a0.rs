// original: 0x00cf87a0 CTaskComplexClimbLadderFully::vf19

/// Probes the ladder state for the given target (passing the object address
/// through a stack slot the comparison snapshots rather than compares), then
/// dispatches on the probe result: a nonzero result builds the 0x11f worker,
/// a zero result the 0xcb worker, returning the builder's result.
///
/// Original: 0x00cf87a0 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf87a0(this: u32, target: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const BUILD_CALLEE: u32 = 2;
        const NONZERO_KIND: u32 = 0x11f;
        const ZERO_KIND: u32 = 0xcb;
        // The original pushes ecx (the object) as a stack slot and passes its
        // address; the contract skips the address and snapshots this word.
        let slot = this;
        let probe: u32 = lf_checker_rt::callee_cdecl!(
            PROBE_CALLEE, u32, target, &slot as *const u32 as u32, 1);
        let kind = if probe != 0 { NONZERO_KIND } else { ZERO_KIND };
        lf_checker_rt::callee_thiscall!(BUILD_CALLEE, u32, this, kind, target)
    }
});
