// original: 0x00c7adc0 CTaskComplexWaitForTime::vf21

/// Count the wait timer down by the frame delta; true when it lapses.
/// The float at `this+0x14` is reduced by the global frame delta,
/// stored back, and the result is 1 iff the remainder is at or below
/// zero. Only the low byte of EAX is set. The stack word is popped
/// but never read.
/// Original: 0x00c7adc0 (thiscall, one ignored stack word).
lf_checker_rt::export!(thiscall, rw_00c7adc0(this: u32, _a0: u32) -> u32 {
    unsafe {
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
        let cur = f32::from_bits(rd32(this.wrapping_add(OFF_TIMER)));
        let dt = f32::from_bits(rd32(lf_checker_rt::relocated(DELTA)));
        let rem = fsub(cur, dt);
        wr32(this.wrapping_add(OFF_TIMER), rem.to_bits());
        if 0.0f32 >= rem { 1 } else { 0 }
    }
});
