// original: 0x00b09cf0 CCamGame::vf6
/// Reset the game camera to its idle state.
///
/// Clears the mode, timer and flag words, copies the shared default word
/// into its slot, runs the shared camera reset step, then sets the idle
/// marker bit and the default zoom of 1.0. Always returns 1.
export!(thiscall, rw_00b09cf0(this_ptr: u32) -> u32 {
    unsafe {
        const SHARED_DEFAULT: u32 = 0x0117_35C4;
        const IDLE_COUNT: u32 = 3;
        const IDLE_TIMER: u32 = 0x7530;
        const DEFAULT_ZOOM_BITS: u32 = 0x3f80_0000;
        let base = this_ptr as usize;
        *((base + 0x1c5) as *mut u8) &= 0xf1;
        *((base + 0x1e4) as *mut u32) = 0;
        *((base + 0x1e8) as *mut u32) = 0;
        *((base + 0x1e0) as *mut u32) = 0;
        *((base + 0x144) as *mut u8) = 0;
        *((base + 0x140) as *mut u32) = IDLE_COUNT;
        *((base + 0x150) as *mut u8) = 0;
        let default = *global::<u32>(SHARED_DEFAULT);
        *((base + 0x1c4) as *mut u8) &= 0xf8;
        *((base + 0x148) as *mut u32) = default;
        *((base + 0x14c) as *mut u32) = IDLE_TIMER;
        callee_thiscall!(1, u32, this_ptr);
        let flags = *((base + 0x1c4) as *const u8);
        *((base + 0x1c5) as *mut u8) &= 0xfe;
        *((base + 0x1c4) as *mut u8) = (flags & 0xc7) | 0x80;
        *((base + 0x1bc) as *mut u32) = DEFAULT_ZOOM_BITS;
        *((base + 0x1d8) as *mut u8) = 0;
        *((base + 0x1d4) as *mut u32) = 0;
        *((base + 0x154) as *mut u32) = 0;
        *((base + 0x1dc) as *mut u32) = 0;
        1
    }
});
