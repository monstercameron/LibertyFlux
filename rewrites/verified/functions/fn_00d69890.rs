// original: 0x00D69890 replay_widget_child_is_active

/// Returns the child active byte at offset 0x1c through the pointer at offset 0x10, or zero for a null pointer.
lf_checker_rt::export!(thiscall, rw_00d69890(this: u32) -> u8 {
    unsafe {
        const PARENT_CHILD_PTR: u32 = 16;
        const CHILD_BYTE: u32 = 28;
        let child = (this.wrapping_add(PARENT_CHILD_PTR) as *const u32).read_unaligned();
        if child == 0 {
            0u8
        } else {
            (child.wrapping_add(CHILD_BYTE) as *const u8).read()
        }
    }
});
