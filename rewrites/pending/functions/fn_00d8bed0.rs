// original: 0x00d8bed0 audio_mode_allows_playback
/// Playback-mode predicate: true while the global audio mode is 1 or 4.
export!(cdecl, rw_00d8bed0() -> u32 {
    unsafe {
        let mode = *global::<u32>(0x0179_BF98);
        ((mode == 1) || (mode == 4)) as u32
    }
});
