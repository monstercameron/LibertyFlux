// original: 0x00D52C30 CCamAnimated::initialize_animation_state

/// Initializes the animation state fields on the object in `this`.
///
/// The routine clears the timer, counters and flags at offsets `0x140` and
/// `0x144` through `0x158`, clears the byte at `0x1b0`, sets the float-shaped
/// state at `0x1b4` to the exact bit pattern for negative one, and clears the
/// word at `0x1b8`. It returns one in the low byte. The object offsets are
/// part of the engine's 32-bit layout; no callees or global state are used.
lf_checker_rt::export!(thiscall, rw_00d52c30(this: u32) -> u8 {
    const TIMER: u32 = 0x140;
    const COUNTER_A: u32 = 0x144;
    const COUNTER_B: u32 = 0x148;
    const SENTINEL: u32 = 0x14c;
    const COUNTER_C: u32 = 0x150;
    const COUNTER_D: u32 = 0x154;
    const COUNTER_E: u32 = 0x158;
    const STATE_FLAG: u32 = 0x1b0;
    const STATE_VALUE: u32 = 0x1b4;
    const STATE_AUX: u32 = 0x1b8;

    unsafe {
        (this.wrapping_add(TIMER) as *mut u32).write_unaligned(0);
        (this.wrapping_add(COUNTER_A) as *mut u32).write_unaligned(0);
        (this.wrapping_add(COUNTER_B) as *mut u32).write_unaligned(0);
        (this.wrapping_add(SENTINEL) as *mut u32).write_unaligned(u32::MAX);
        (this.wrapping_add(COUNTER_C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(COUNTER_D) as *mut u32).write_unaligned(0);
        (this.wrapping_add(COUNTER_E) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_FLAG) as *mut u8).write(0);
        (this.wrapping_add(STATE_VALUE) as *mut u32).write_unaligned(0xbf80_0000);
        (this.wrapping_add(STATE_AUX) as *mut u32).write_unaligned(0);
    }

    1
});
