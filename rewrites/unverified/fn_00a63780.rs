// original: 0x00A63780 contains_task_selector_value

/// Test whether `selector` equals one of the three selector words at
/// `CPedIntelligence + 0x2d0`, `+0x2d4`, or `+0x2d8`. The result is a
/// one-byte boolean in AL; the upper return-register bytes are not part of
/// the result. `this` is in ECX, `selector` is one stack word, and the
/// callee removes it.
lf_checker_rt::export!(thiscall, rw_00a63780(this: u32, selector: u32) -> u32 {
    unsafe {
        const SELECTORS: u32 = 0x2d0;
        const SELECTOR_COUNT: u32 = 3;
        for index in 0..SELECTOR_COUNT {
            if (this.wrapping_add(SELECTORS + index * 4) as *const u32).read_unaligned() == selector {
                return 1;
            }
        }
        0
    }
});
