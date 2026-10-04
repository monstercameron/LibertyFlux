// original: 0x0097C080 audio_cursor_below_limit
/// Test whether the cursor plus a fixed span is still below the live limit.
///
/// Returns the comparison in the low byte, keeping the sum's upper bytes the
/// way the original's flag-set does. thiscall(obj).
export!(thiscall, rw_s103_97c080(obj: *const u8) -> u32 {
    unsafe {
        let s = (*(((obj as usize) + 0x13C) as *const u32)).wrapping_add(0x12C);
        let limit = *global::<u32>(0x11735B4);
        (s & !0xFF) | ((s < limit) as u32)
    }
});
