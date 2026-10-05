// original: 0x008C8860 stream_state_clear
/// Reset the streaming state machine to its initial (all-zero) state.
///
/// Clears the two state dwords and the active flag. Returns nothing.
/// Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008c8860() -> u32 {
    unsafe {
        *lf_checker_rt::global::<u32>(0x1172f54) = 0;
        *lf_checker_rt::global::<u32>(0x11730f8) = 0;
        *lf_checker_rt::global::<u8>(0x1172f53) = 0;
        0
    }
});
