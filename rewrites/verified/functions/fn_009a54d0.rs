// original: 0x009a54d0 script_audio_lookup_voice
/// Original 0x009a54d0 (unnamed): look up a script-audio voice.
///
/// Returns -1 when the owner word at +8 is clear. Otherwise reads the
/// signed voice index at +0x2bd0: a nonzero mode byte returns it at once
/// and latches the flag at +0x2bd5; a negative index skips straight to the
/// notify helper; otherwise the voice pointer is resolved and a live voice
/// is released through its own destructor first, a null one through the
/// idle sub-object. Returns the index.
export!(thiscall, rw_009a54d0(this_: u32, arg: u32) -> u32 {
    if unsafe { ((this_ + 8) as *const u32).read() } == 0 {
        return 0xffff_ffff;
    }
    let idx = unsafe { ((this_ + 0x2bd0) as *const u16).read() as i16 as i32 };
    if arg & 0xff != 0 {
        unsafe { ((this_ + 0x2bd5) as *mut u8).write(1) };
        return idx as u32;
    }
    if idx < 0 {
        callee_thiscall!(2, u32, this_);
        return idx as u32;
    }
    let t = unsafe {
        (this_
            .wrapping_add((idx as u32).wrapping_mul(0x70))
            .wrapping_add(0x15ec) as *const u32)
            .read()
    };
    let u = unsafe {
        (this_.wrapping_add(t.wrapping_mul(4)).wrapping_add(0x2df0) as *const u32)
            .read()
    };
    if u != 0 {
        callee_thiscall!(1, u32, u.wrapping_add(0x570));
    } else {
        callee_thiscall!(1, u32, this_.wrapping_add(0x2be0));
    }
    callee_thiscall!(2, u32, this_);
    idx as u32
});
