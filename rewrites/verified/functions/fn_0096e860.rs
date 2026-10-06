// original: 0x0096e860 audio_slot_array_release (proposed)

/// Release every live entry of a 64-slot pointer array at +0x2A30.
///
/// For each of the 64 slots, when the stored pointer is nonzero the release
/// callee runs on the slot address and the slot is cleared. The callee takes
/// one stack word and cleans it (stdcall).
/// Original: 0x0096E860 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0096e860(this: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x2A30;
        const COUNT: u32 = 0x40;
        const RELEASE: u32 = 1;
        for i in 0..COUNT {
            let slot = this.wrapping_add(BASE).wrapping_add(i * 4);
            if (slot as *const u32).read_unaligned() != 0 {
                lf_checker_rt::callee_stdcall!(RELEASE, u32, slot);
                (slot as *mut u32).write_unaligned(0);
            }
        }
        0
    }
});
