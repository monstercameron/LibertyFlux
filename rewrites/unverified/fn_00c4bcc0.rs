// original: 0x00C4BCC0 camera_cinematic_update_parameter_state

/// Update the parameter bounds, flag mask, copied time fields, and target
/// duration difference in a cinematic camera state object.
///
/// The receiver is in ECX. It copies the dword at 0x98 to 0x9c, clears the
/// low five bits of the byte at 0xa0, copies the dword at 0x84 to 0x80, and
/// stores `f32(0x4c) - f32(0x50)` at 0x78. EAX retains the copied dword from
/// 0x84. The float subtraction preserves the original left-minus-right order.
lf_checker_rt::export!(thiscall, rw_c4bcc0(this: u32) -> u32 {
    const LEFT_DURATION: u32 = 0x4c;
    const RIGHT_DURATION: u32 = 0x50;
    const DURATION_DIFFERENCE: u32 = 0x78;
    const COPIED_TIME_SOURCE: u32 = 0x84;
    const COPIED_TIME_DESTINATION: u32 = 0x80;
    const COPIED_PARAMETER_SOURCE: u32 = 0x98;
    const COPIED_PARAMETER_DESTINATION: u32 = 0x9c;
    const FLAGS: u32 = 0xa0;
    const KEEP_HIGH_FLAG_BITS: u8 = 0xe0;

    let left_bits = unsafe {
        (this.wrapping_add(LEFT_DURATION) as *const u32).read_unaligned()
    };
    let right_bits = unsafe {
        (this.wrapping_add(RIGHT_DURATION) as *const u32).read_unaligned()
    };
    let difference = core::hint::black_box(f32::from_bits(left_bits))
        - core::hint::black_box(f32::from_bits(right_bits));
    let parameter = unsafe {
        (this.wrapping_add(COPIED_PARAMETER_SOURCE) as *const u32).read_unaligned()
    };
    let time = unsafe {
        (this.wrapping_add(COPIED_TIME_SOURCE) as *const u32).read_unaligned()
    };
    let flags = unsafe { (this.wrapping_add(FLAGS) as *const u8).read_unaligned() };

    unsafe {
        (this.wrapping_add(DURATION_DIFFERENCE) as *mut u32)
            .write_unaligned(difference.to_bits());
        (this.wrapping_add(COPIED_PARAMETER_DESTINATION) as *mut u32)
            .write_unaligned(parameter);
        (this.wrapping_add(FLAGS) as *mut u8)
            .write_unaligned(flags & KEEP_HIGH_FLAG_BITS);
        (this.wrapping_add(COPIED_TIME_DESTINATION) as *mut u32).write_unaligned(time);
    }
    time
});
