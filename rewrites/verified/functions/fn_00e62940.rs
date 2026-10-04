// original: 0x00e62940 audio_zero3_emit_tag
/// Clear three audio state words, then register the module tag.
///
/// Same shape as the four-word variant: zeroes the state block, invokes the
/// shared tag sink (cdecl/1), returns the sink's answer.
export!(cdecl, rw_00e62940() -> u32 {
    unsafe {
        const STATE: u32 = 0x0116_1818;
        const TAG: u32 = 0x00E7_10C0;
        let state = global::<u32>(STATE);
        *state = 0;
        *state.add(1) = 0;
        *state.add(2) = 0;
        callee_cdecl!(1, u32, relocated(TAG))
    }
});
