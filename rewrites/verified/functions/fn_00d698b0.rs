// original: 0x00D698B0 replay_montage_get_open_state

/// Returns the child open-state byte at offset 0x48 through the pointer at offset 0x0c, or zero for a null pointer.
lf_checker_rt::export!(thiscall, rw_00d698b0(this: u32) -> u8 {
    unsafe {
        const PARENT_CHILD_PTR: u32 = 12;
        const CHILD_BYTE: u32 = 72;
        let child = (this.wrapping_add(PARENT_CHILD_PTR) as *const u32).read_unaligned();
        if child == 0 {
            0u8
        } else {
            (child.wrapping_add(CHILD_BYTE) as *const u8).read()
        }
    }
});
