// original: 0x00A3C920 vehicle_reset_motion_fields

/// Reset the receiver's motion fields after invoking its no-argument helper.
/// The helper runs first with the receiver as `this`; the routine then clears
/// the word at `+0x80` and stores the bit pattern for `1.0f32` at `+0x8c`.
/// The original is a thiscall method with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3c920(this: u32) -> () {
    const MOTION_STATE: u32 = 0x80;
    const UNIT_SCALE: u32 = 0x8c;
    const RESET_HELPER: u32 = 1;
    const ONE_BITS: u32 = 0x3f80_0000;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(RESET_HELPER, u32, this);
        (this.wrapping_add(MOTION_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(UNIT_SCALE) as *mut u32).write_unaligned(ONE_BITS);
    }
});
