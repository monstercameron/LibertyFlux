// original: 0x00B27E10 update_blend_timing_state

/// Update the blend timing key at +0x3c. The prior key at +0x40 and the unit
/// transition weight at +0x44 are updated only when the new key differs and
/// the previous key is not the `u32::MAX` uninitialized sentinel. Returns no
/// meaningful value; the original uses a thiscall receiver and one stack word.
lf_checker_rt::export!(thiscall, rw_b27e10(this: u32, next_key: u32) -> u32 {
    unsafe {
        const CURRENT_KEY: u32 = 0x3c;
        const PRIOR_KEY: u32 = 0x40;
        const TRANSITION_WEIGHT: u32 = 0x44;
        const UNINITIALIZED: u32 = u32::MAX;
        const UNIT_WEIGHT: u32 = 0x3f80_0000;
        let current_key = (this.wrapping_add(CURRENT_KEY) as *const u32).read_unaligned();
        (this.wrapping_add(CURRENT_KEY) as *mut u32).write_unaligned(next_key);
        if current_key != next_key && current_key != UNINITIALIZED {
            (this.wrapping_add(PRIOR_KEY) as *mut u32).write_unaligned(current_key);
            (this.wrapping_add(TRANSITION_WEIGHT) as *mut u32).write_unaligned(UNIT_WEIGHT);
        }
        next_key
    }
});
