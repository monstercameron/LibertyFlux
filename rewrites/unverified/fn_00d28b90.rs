// original: 0x00d28b90 target_slot_find_1based (proposed)

/// Find a wanted pointer among the 8 target slots, 1-based.
///
/// Each of the 8 slots holds a pointer at `this + 0x34 + i * 0x40`. Returns
/// `i + 1` for the first slot equal to `wanted`, or 0 when none matches. A
/// null `wanted` never matches, even against null slots.
///
/// Original: 0x00D28B90 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d28b90(this: u32, wanted: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x34;
        const SLOT_STRIDE: u32 = 0x40;
        const SLOT_COUNT: u32 = 8;
        let mut idx = 0u32;
        let mut slot = this + SLOT_BASE;
        loop {
            if wanted != 0 && unsafe { (slot as *const u32).read_unaligned() } == wanted {
                return idx + 1;
            }
            idx += 1;
            slot += SLOT_STRIDE;
            if idx >= SLOT_COUNT {
                return 0;
            }
        }
    }
});
