// original: 0x00D696C0 get_replay_bar_time_factor

/// Returns the float at child offset 0x3c through the object pointer at offset 0x14, or positive zero when that child pointer is null.
lf_checker_rt::export!(thiscall, rw_00d696c0(this: u32) -> f32 {
    unsafe {
        const CHILD_POINTER: u32 = 0x14;
        const TIME_FACTOR: u32 = 0x3c;
        let child = (this.wrapping_add(CHILD_POINTER) as *const u32).read_unaligned();
        if child == 0 {
            0.0f32
        } else {
            (child.wrapping_add(TIME_FACTOR) as *const f32).read_unaligned()
        }
    }
});
