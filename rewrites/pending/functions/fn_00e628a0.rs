// original: 0x00e628a0 audio_zero4_emit_tag
/// Clear four audio state words, then register the module tag.
///
/// Zeroes the four-word state block before invoking the shared tag sink
/// (cdecl/1) with this module's tag address. Returns the sink's answer.
export!(cdecl, rw_00e628a0() -> u32 {
    unsafe {
        const STATE: u32 = 0x0116_1800;
        const TAG: u32 = 0x00E7_10A0;
        let state = global::<u32>(STATE);
        *state = 0;
        *state.add(1) = 0;
        *state.add(2) = 0;
        *state.add(3) = 0;
        callee_cdecl!(1, u32, relocated(TAG))
    }
});
