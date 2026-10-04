// original: 0x00cbbb90 CTaskSimpleMoveGoToPointOnRoute::vf1
/// Clamp a speed argument into `[0, 3]` and store it (vf1, leaf).
///
/// Stores `a0` clamped below at `0.0` and above at `3.0` (the constant
/// from file address `0x00FE8A94`) into `[this + 0x10]` (thiscall, two
/// stack arguments; `a1` is unread). An unordered (NaN) input passes
/// through unchanged: neither bound compares greater. No return value
/// (the original never writes eax), no calls.
lf_checker_rt::export!(thiscall, rw_00cbbb90(this: u32, a0: u32, _a1: u32) -> u32 {
    unsafe {
        /// Slot receiving the clamped speed.
        const SPEED_OFF: u32 = 0x10;
        /// Upper clamp (constant from file address 0x00FE8A94).
        const MAX_SPEED: f32 = 3.0;
        let x = f32::from_bits(a0);
        let clamped = if x < 0.0 {
            0.0
        } else if x > MAX_SPEED {
            MAX_SPEED
        } else {
            x
        };
        ((this + SPEED_OFF) as *mut u32).write_unaligned(clamped.to_bits());
        0
    }
});
