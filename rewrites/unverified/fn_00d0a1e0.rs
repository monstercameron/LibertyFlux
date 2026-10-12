// original: 0x00D0A1E0 CTaskComplexSeperate::vf19

/// Steer a separate task toward its waypoint, then run its tail step.
///
/// Unless the steered bit (bit 0 of `+0xb8`) is already set, runs the steering
/// solver (patched cdecl callee) over the waypoint data and sets the bit on a
/// nonzero low byte, returning zero on failure. Then runs the follow step
/// (patched thiscall callee); a nonzero done byte at `+0x98` runs the tail
/// step (planted stub callee through the table at `+0xb0`) and returns its
/// answer, otherwise returns zero.
///
/// Original: 0x00D0A1E0 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d0a1e0(this: u32, a1: u32) -> u32 {
    unsafe {
        const SOLVE: u32 = 1;
        const FOLLOW: u32 = 2;
        const STEP_SLOT: u32 = 0x04;
        const GAIN_SLOT: u32 = 0x01053CC0;
        let gain = lf_checker_rt::global::<u32>(GAIN_SLOT).read();
        if ((this + 0xb8) as *const u8).read() & 1 == 0 {
            let facing = ((a1 + 0x20) as *const u32).read_unaligned();
            let ok = lf_checker_rt::callee_cdecl!(SOLVE, u32, a1,
                ((this + 0xac) as *const u32).read_unaligned(),
                facing.wrapping_add(0x30), gain, this + 0x20, this + 0xb4, 0);
            if (ok & 0xFF) == 0 {
                return 0;
            }
            ((this + 0xb8) as *mut u8).write(((this + 0xb8) as *const u8).read() | 1);
        }
        lf_checker_rt::callee_thiscall!(FOLLOW, u32, this, a1, 0);
        if ((this + 0x98) as *const u8).read() == 0 {
            return 0;
        }
        let tail = ((this + 0xb0) as *const u32).read_unaligned();
        let step: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(
                ((((tail) as *const u32).read_unaligned() + STEP_SLOT) as *const u32)
                .read_unaligned() as usize);
        step(tail)
    }
});
