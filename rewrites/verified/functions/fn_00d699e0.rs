// original: 0x00D699E0 replay_tool_overlay_set_child_state

/// If the child pointer at offset 0x0c is non-null, stores the low byte of `state` at child offset 0x18.
lf_checker_rt::export!(thiscall, rw_00d699e0(this: u32, state: u32) -> () {
    unsafe {
        const CHILD_POINTER: u32 = 0x0c;
        const CHILD_STATE: u32 = 0x18;
        let child = (this.wrapping_add(CHILD_POINTER) as *const u32).read_unaligned();
        if child != 0 {
            (child.wrapping_add(CHILD_STATE) as *mut u8).write(state as u8);
        }
    }
});
