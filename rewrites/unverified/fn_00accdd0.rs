// original: 0x00ACCDD0 audio_forward_9_to_engine (proposed)

/// Forward nine argument words to the audio engine helper.
///
/// Copies all nine stack words (two of them with floating-point moves, but
/// the callee receives the same bits either way) into a fresh call to the
/// engine helper (cdecl/9) and returns its answer (cdecl/9).
lf_checker_rt::export!(cdecl, rw_00accdd0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        const ENGINE: u32 = 1;
        lf_checker_rt::callee_cdecl!(ENGINE, u32, a0, a1, a2, a3, a4, a5, a6, a7, a8)
    }
});
