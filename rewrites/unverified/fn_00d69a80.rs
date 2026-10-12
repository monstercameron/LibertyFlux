// original: 0x00D69A80 any_replay_ui_child_has_state_flag

/// Returns true if every child pointer at offsets 0, 4, 8, 0x0c, and 0x10 is non-null and at least one child's byte at offset 0x18 is nonzero. A null pointer returns false.
lf_checker_rt::export!(thiscall, rw_00d69a80(this: u32) -> u8 {
    unsafe {
        const CHILD_POINTERS: [u32; 5] = [0x00, 0x04, 0x08, 0x0c, 0x10];
        const CHILD_STATE: u32 = 0x18;
        for field in CHILD_POINTERS {
            let child = (this.wrapping_add(field) as *const u32).read_unaligned();
            if child == 0 {
                return 0;
            }
            if (child.wrapping_add(CHILD_STATE) as *const u8).read() != 0 {
                return 1;
            }
        }
        0
    }
});
