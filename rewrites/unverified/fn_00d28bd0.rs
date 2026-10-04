// original: 0x00d28bd0 target_slot_find_index (proposed)

/// Find a wanted pointer among the 8 target slots, 0-based.
///
/// Same 8 slots as the 1-based search (`this + 0x34 + i * 0x40`), but returns
/// the 0-based index `i`, or -1 (all bits set) when nothing matches. A null
/// `wanted` never matches.
///
/// Original: 0x00D28BD0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d28bd0(this: u32, wanted: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x34;
        const SLOT_STRIDE: u32 = 0x40;
        const SLOT_COUNT: u32 = 8;
        const MISS: u32 = 0xffff_ffff;
        let mut idx = 0u32;
        let mut slot = this + SLOT_BASE;
        loop {
            if wanted != 0 && unsafe { (slot as *const u32).read_unaligned() } == wanted {
                return idx;
            }
            idx += 1;
            slot += SLOT_STRIDE;
            if idx >= SLOT_COUNT {
                return MISS;
            }
        }
    }
});
