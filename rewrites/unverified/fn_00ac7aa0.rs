// original: 0x00AC7AA0 decrement_stream_timer

/// Subtract a timestep from the object's timer and clamp negative results to
/// zero.
///
/// The timer is the `f32` at `this + 0x15c`. Subtraction is performed in the
/// original order (stored timer minus argument). The x86 ordered comparison
/// keeps NaN values and clamps only a negative result. This thiscall method
/// removes its one stack argument.
lf_checker_rt::export!(thiscall, rw_00ac7aa0(this: u32, elapsed: f32) -> () {
    const TIMER_OFFSET: u32 = 0x15c;

    unsafe {
        let timer = (this.wrapping_add(TIMER_OFFSET)) as *mut f32;
        let remaining = core::hint::black_box(timer.read_unaligned())
            - core::hint::black_box(elapsed);
        timer.write_unaligned(if remaining < 0.0 { 0.0 } else { remaining });
    }
});
