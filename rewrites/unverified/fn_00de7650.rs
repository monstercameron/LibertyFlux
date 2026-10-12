// original: 0x00DE7650 montage_position_bar_set_child_state

/// Visit the seven child pointers beginning at `this + 0x204`. For each child,
/// call virtual method `+0x120` with zero when the low byte of `state_flags`
/// is nonzero and one when that byte is zero. Return the last call's 32-bit
/// result. This is a `thiscall` method with one stack argument.
lf_checker_rt::export!(thiscall, rw_00DE7650(this: u32, state_flags: u32) -> u32 {
    const CHILD_POINTERS: u32 = 0x204;
    const CHILD_COUNT: u32 = 7;
    const CALLEE_SET_STATE: u32 = 1;

    let low_state_byte = state_flags as u8;
    let call_argument = if low_state_byte == 0 { 1u32 } else { 0u32 };
    let mut last_result = 0u32;
    for child_index in 0..CHILD_COUNT {
        let child_slot = this.wrapping_add(CHILD_POINTERS + child_index * 4);
        let child = unsafe { (child_slot as *const u32).read_unaligned() };
        last_result = lf_checker_rt::callee_thiscall!(CALLEE_SET_STATE, u32, child, call_argument);
    }
    last_result
});
