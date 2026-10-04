// original: 0x0097C0B0 audio_flag_130_is_set
/// Test whether the flag word at +0x130 is non-zero. thiscall(obj).
export!(thiscall, rw_s103_97c0b0(obj: *const u8) -> u32 {
    unsafe { (*(((obj as usize) + 0x130) as *const u32) != 0) as u32 }
});
