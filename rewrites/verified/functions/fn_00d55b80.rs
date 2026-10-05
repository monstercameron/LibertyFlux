// original: 0x00d55b80 CCamWasted::vf7

/// Notify the wasted camera's target, then reset the two global blend factors.
///
/// `this` points to the object. The target pointer at `TARGET` is read; when
/// non-null the notify routine runs with the target in ECX and the slot
/// address (`this + TARGET`) as its stack argument. Either way the two global
/// blend factors `BLEND_A` and `BLEND_B` are then reset to 1.0. Returns 1 in
/// AL (upper EAX passes through, so only AL is compared).
///
/// Original: 0x00d55b80 (thiscall, no stack arguments, one conditional call).
lf_checker_rt::export!(thiscall, rw_00d55b80(this: u32) -> u32 {
    unsafe {
        /// Slot holding the target pointer (and passed as the call argument).
        const TARGET: u32 = 0x144;
        /// Global blend factors reset to one.
        const BLEND_A: u32 = 0x01032350;
        const BLEND_B: u32 = 0x0103234c;
        /// Bit pattern of 1.0f.
        const ONE: u32 = 0x3f800000;
        /// Target notify routine (intercepted; thiscall, one stack argument).
        const NOTIFY: u32 = 1;
        let target = ((this + TARGET) as *const u32).read_unaligned();
        if target != 0 {
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, target, this.wrapping_add(TARGET));
        }
        (lf_checker_rt::global::<u32>(BLEND_A) as *mut u32).write_unaligned(ONE);
        (lf_checker_rt::global::<u32>(BLEND_B) as *mut u32).write_unaligned(ONE);
        1
    }
});
