// original: 0x00C321B0 vehicle_speed_in_handling_range

/// Return one in AL when the activity bit in the vehicle record is set and
/// its speed value is strictly greater than zero and strictly less than five;
/// otherwise return zero. Unordered float comparisons, including NaNs, take
/// the false path. The original is a thiscall method with no stack arguments
/// and reads the speed as an `f32`.
lf_checker_rt::export!(thiscall, rw_00c321b0(this: *const u8) -> u8 {
    const ACTIVITY_OFFSET: usize = 0x118;
    const SPEED_OFFSET: usize = 0x14ac;
    const ACTIVE_BIT: u8 = 1;
    const MIN_SPEED: f32 = 0.0;
    const MAX_SPEED: f32 = 5.0;

    let activity = unsafe { this.add(ACTIVITY_OFFSET).read() };
    let speed_bits = unsafe {
        this.add(SPEED_OFFSET)
            .cast::<u32>()
            .read_unaligned()
    };
    let speed = f32::from_bits(speed_bits);
    u8::from(activity & ACTIVE_BIT != 0 && speed > MIN_SPEED && speed < MAX_SPEED)
});
