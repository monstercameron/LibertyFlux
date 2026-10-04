// original: 0x00d28fd0 target_slot_aux_lookup (proposed)

/// Look up the auxiliary word for a wanted pointer among the 8 target slots.
///
/// Searches the slots at `this + 0x34 + i * 0x40` for `wanted` (a null
/// `wanted` matches the first null slot) and returns the parallel word at
/// `this + 0x250 + i * 4`, or 0 when no slot matches.
///
/// Original: 0x00D28FD0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d28fd0(this: u32, wanted: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x34;
        const SLOT_STRIDE: u32 = 0x40;
        const SLOT_COUNT: u32 = 8;
        const AUX_BASE: u32 = 0x250;
        let mut idx = 0u32;
        let mut slot = this + SLOT_BASE;
        loop {
            if unsafe { (slot as *const u32).read_unaligned() } == wanted {
                return unsafe { ((this + AUX_BASE + idx * 4) as *const u32).read_unaligned() };
            }
            idx += 1;
            slot += SLOT_STRIDE;
            if idx >= SLOT_COUNT {
                return 0;
            }
        }
    }
});
