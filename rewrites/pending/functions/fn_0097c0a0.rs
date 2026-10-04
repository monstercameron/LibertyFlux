// original: 0x0097C0A0 audio_flag_134_is_set
/// Test whether the flag word at +0x134 is non-zero. thiscall(obj).
export!(thiscall, rw_s103_97c0a0(obj: *const u8) -> u32 {
    unsafe { (*(((obj as usize) + 0x134) as *const u32) != 0) as u32 }
});
