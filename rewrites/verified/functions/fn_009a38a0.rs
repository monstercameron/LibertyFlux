// original: 0x009a38a0 audio_entity_latch_config
/// Original 0x009a38a0 (unnamed): latch a global dword into the object once.
///
/// When the flag byte at +0x89 is clear, copies the global dword into the
/// slot at +0x18, then sets the flag in all cases. The original leaves no
/// meaningful value in EAX on the already-set path, so no return is compared.
export!(thiscall, rw_009a38a0(this_: u32) -> u32 {
    let flag = unsafe { ((this_ + 0x89) as *const u8).read() };
    if flag == 0 {
        let g = unsafe { (relocated(0x011735B4) as *const u32).read() };
        unsafe { ((this_ + 0x18) as *mut u32).write(g) };
    }
    unsafe { ((this_ + 0x89) as *mut u8).write(1) };
    0
});
