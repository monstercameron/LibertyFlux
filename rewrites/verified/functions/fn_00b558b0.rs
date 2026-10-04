// original: 0x00b558b0 crFrameFilterBoneMask::vf3
/// Gate a bone-index lookup on the mode byte (frame-filter vtable slot 3).
///
/// Modes 0 and 1, and mode 0x15, consult the helper on the inner object at
/// +0x0c with the u16 index; any other mode, a null inner object, or a
/// non-zero helper answer yields 1. A zero helper answer is returned as-is
/// (its full EAX, whose low byte is then 0).
export!(thiscall, rw_00b558b0(this: u32, mode: u32, index: u32, _unused: u32) -> u32 {
    unsafe {
        let m = (mode & 0xff) as u8;
        if m > 1 && m != 0x15 {
            return 1;
        }
        let inner = ((this + 0x0c) as *const u32).read();
        if inner == 0 {
            return 1;
        }
        let answer: u32 = callee_cdecl!(1, u32, inner, index & 0xffff);
        if answer & 0xff != 0 {
            1
        } else {
            answer
        }
    }
});
