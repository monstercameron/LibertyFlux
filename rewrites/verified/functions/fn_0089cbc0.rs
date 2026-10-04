// original: 0x0089cbc0 reset_audio_globals_and_release
/// Reset eight global words to zero, then release the saved global pointer
/// through the cdecl/1 helper. Returns the helper's answer.
export!(cdecl, rw_0089cbc0() -> u32 {
    unsafe {
        const FIRST: u32 = 0x115F824;
        const COUNT: u32 = 8;
        let saved: u32 = *global::<u32>(0x115DEB0);
        let mut i = 0u32;
        while i < COUNT {
            *global::<u32>(FIRST + i * 4) = 0;
            i += 1;
        }
        callee_cdecl!(1, u32, saved)
    }
});
