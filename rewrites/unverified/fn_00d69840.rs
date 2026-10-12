// original: 0x00D69840 get_replay_montage_open_flag

/// Returns the child open-flag byte at offset 0x48 through the pointer at offset 8, or zero for a null pointer.
lf_checker_rt::export!(thiscall, rw_00d69840(this: u32) -> u8 {
    unsafe {
        const PARENT_CHILD_PTR: u32 = 8;
        const CHILD_BYTE: u32 = 72;
        let child = (this.wrapping_add(PARENT_CHILD_PTR) as *const u32).read_unaligned();
        if child == 0 {
            0u8
        } else {
            (child.wrapping_add(CHILD_BYTE) as *const u8).read()
        }
    }
});
