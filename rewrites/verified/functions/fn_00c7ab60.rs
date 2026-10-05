// original: 0x00c7ab60 CTaskComplexDriveWanderForTime::vf21

/// Count the wander timer down by the frame delta; true when it lapses.
/// When the owner's link at `[owner+0xb30]` is null the wait is already
/// over (returns 1). Else the float at `this+0x14` is reduced by the
/// global frame delta, stored back, and the result is 1 iff the
/// remainder is at or below zero. Only the low byte of EAX is set.
/// Original: 0x00c7ab60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c7ab60(this: u32, owner: u32) -> u32 {
    unsafe {
        const OFF_OWNER: u32 = 0xb30;
        const OFF_TIMER: u32 = 0x14;
        const DELTA: u32 = 0x011735BC;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let link = rd32(owner.wrapping_add(OFF_OWNER));
        if link == 0 {
            return 1;
        }
        let cur = f32::from_bits(rd32(this.wrapping_add(OFF_TIMER)));
        let dt = f32::from_bits(rd32(lf_checker_rt::relocated(DELTA)));
        let rem = fsub(cur, dt);
        wr32(this.wrapping_add(OFF_TIMER), rem.to_bits());
        if 0.0f32 >= rem { 1 } else { 0 }
    }
});
