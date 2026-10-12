// original: 0x00D69A50 toggle_replay_montage_state

/// When the child pointer at offset 0x10 exists, clears its byte at 0x1c, toggles its byte at 0x1d between zero and one, and mirrors that new byte to the replay montage state global.
lf_checker_rt::export!(thiscall, rw_00d69a50(this: u32) -> () {
    unsafe {
        const CHILD_POINTER: u32 = 0x10;
        const RESET_BYTE: u32 = 0x1c;
        const TOGGLE_BYTE: u32 = 0x1d;
        const OPEN_STATE_GLOBAL: u32 = 0x01030b9f;
        let child = (this.wrapping_add(CHILD_POINTER) as *const u32).read_unaligned();
        if child != 0 {
            let state = child.wrapping_add(TOGGLE_BYTE) as *mut u8;
            let old = state.read();
            (child.wrapping_add(RESET_BYTE) as *mut u8).write(0);
            let new = u8::from(old == 0);
            state.write(new);
            lf_checker_rt::global::<u8>(OPEN_STATE_GLOBAL).write(new);
        }
    }
});
