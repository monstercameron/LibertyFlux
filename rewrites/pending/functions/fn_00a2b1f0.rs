// original: 0x00a2b1f0 NativeImpl_SET_PLAYER_GROUP_TO_FOLLOW_ALWAYS

/// Toggle the "follow always" player-group flag on the ped's extension.
/// `this` is the player ped; the flag lives in bit 0x200 of the word at
/// `+0x3D0` of the inner block (`[this+0x228] + 0x70`, or address zero when
/// the extension pointer is null, which faults on the read-modify-write
/// exactly like the original). A zero low byte of `enable` clears the bit
/// and returns the inner pointer; otherwise the bit is set and the request
/// is forwarded to the group worker (callee 1, thiscall with this and
/// `(1, 0, 1)`), whose answer is returned.
/// Original: 0x00a2b1f0 (thiscall, one stack byte).
lf_checker_rt::export!(thiscall, rw_00a2b1f0(this: u32, enable: u32) -> u32 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
        const EXT: u32 = 0x228;
        const INNER: u32 = 0x70;
        const FLAGS: u32 = 0x3d0;
        const FOLLOW_ALWAYS: u32 = 0x200;
        const WORKER: u32 = 1;
        let raw = rd32(this.wrapping_add(EXT));
        let inner = if raw == 0 { 0 } else { raw.wrapping_add(INNER) };
        let slot = inner.wrapping_add(FLAGS);
        if (enable as u8) == 0 {
            wr32(slot, rd32(slot) & !FOLLOW_ALWAYS);
            inner
        } else {
            wr32(slot, rd32(slot) | FOLLOW_ALWAYS);
            lf_checker_rt::callee_thiscall!(WORKER, u32, this, 1u32, 0u32, 1u32)
        }
    }
});
