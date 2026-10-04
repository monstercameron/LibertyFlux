// original: 0x00982180 audio_visit_voice_array
/// Original 0x00982180 (unnamed): fan out over the voice array.
///
/// Looks up the voice-bank header through the global audio manager; when
/// present with a nonzero voice count, visits that many 0x360-byte records
/// starting at the array in +0x6f28. Void; calls are the behaviour.
export!(thiscall, rw_00982180(this_: u32) -> u32 {
    let h = callee_thiscall!(1, u32, relocated(0x0115D9A0), relocated(0x00E8D8C4));
    if h == 0 {
        return 0;
    }
    let n = unsafe { ((h + 0xa) as *const u8).read() };
    if n == 0 {
        return 0;
    }
    let base = unsafe { ((this_ + 0x6f28) as *const u32).read() };
    let mut i = 0u32;
    while i < n as u32 {
        callee_thiscall!(2, u32, base.wrapping_add(i.wrapping_mul(0x360)));
        i += 1;
    }
    0
});
