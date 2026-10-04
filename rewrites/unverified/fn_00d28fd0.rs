// original: 0x00d28fd0 target_slot_find_nullable (proposed)

/// Find a wanted pointer among the 8 target slots, null matching null.
///
/// Same layout as the other slot searches (`this + 0x34 + i * 0x40`, 1-based
/// result with 0 on miss), except there is no null guard: a null `wanted`
/// matches the first null slot.
///
/// Original: 0x00D28FD0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d28fd0(this: u32, wanted: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x34;
        const SLOT_STRIDE: u32 = 0x40;
        const SLOT_COUNT: u32 = 8;
        let mut idx = 0u32;
        let mut slot = this + SLOT_BASE;
        loop {
            if unsafe { (slot as *const u32).read_unaligned() } == wanted {
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
