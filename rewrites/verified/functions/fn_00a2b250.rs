// original: 0x00a2b250 NativeImpl_SET_PLAYER_GROUP_TO_FOLLOW_NEVER

/// Toggle the "follow never" player-group flag on the ped's extension.
/// Mirror of `rw_00a2b1f0` for bit 0x400. The three pushed words land as
/// the worker's arguments in reverse push order, i.e. `(0, 0, 1)`, on the
/// set path.
/// Original: 0x00a2b250 (thiscall, one stack byte).
lf_checker_rt::export!(thiscall, rw_00a2b250(this: u32, enable: u32) -> u32 {
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
        const FOLLOW_NEVER: u32 = 0x400;
        const WORKER: u32 = 1;
        let raw = rd32(this.wrapping_add(EXT));
        let inner = if raw == 0 { 0 } else { raw.wrapping_add(INNER) };
        let slot = inner.wrapping_add(FLAGS);
        if (enable as u8) == 0 {
            wr32(slot, rd32(slot) & !FOLLOW_NEVER);
            inner
        } else {
            wr32(slot, rd32(slot) | FOLLOW_NEVER);
            lf_checker_rt::callee_thiscall!(WORKER, u32, this, 0u32, 0u32, 1u32)
        }
    }
});
