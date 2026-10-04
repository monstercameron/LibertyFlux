// original: 0x00d8e300 audio_global_release_and_clear
/// Releases the global audio handle through the free helper (when set) and
/// clears the slot. Returns the helper's answer, or 0 when already clear.
export!(cdecl, rw_00d8e300() -> u32 {
    unsafe {
        let slot = global::<u32>(0x0179_F934);
        let r = if *slot != 0 { callee_cdecl!(1, u32, *slot) } else { 0 };
        *slot = 0;
        r
    }
});
