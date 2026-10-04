// original: 0x00a1f5c0 CCamFollowPed::vf5 (symbols)

/// Clamps the follow distance into range and refreshes the camera.
///
/// The float at `this + VAL_OFF` is clamped into [`LO`, `HI`]
/// (85.0/45.0): above `HI` it becomes `HI`, below `LO` it becomes `LO`,
/// otherwise it is left alone (a NaN is left alone). The mode byte at
/// `this + MODE_OFF` is copied to a global out-slot, and when the global
/// selector equals `WANT` a worker callee runs on `this + ARG_OFF`.
/// Always returns 1 (`al`; the callee's return value is discarded, so the
/// upper bytes of `eax` are the callee's leftovers).
///
/// Original: 0x00a1f5c0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1f5c0(this: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        const VAL_OFF: u32 = 0x60;
        const MODE_OFF: u32 = 0x360;
        const ARG_OFF: u32 = 0x10;
        const HI: f32 = f32::from_bits(0x42aa_0000); // 85.0
        const LO: f32 = f32::from_bits(0x4234_0000); // 45.0
        const SELECTOR: u32 = 0x011d_6fd4;
        const OUT_SLOT: u32 = 0x0103_bff8;
        const WANT: u32 = 2;
        let v = f32::from_bits(((this + VAL_OFF) as *const u32).read_unaligned());
        if v > HI {
            ((this + VAL_OFF) as *mut u32).write_unaligned(HI.to_bits());
        } else if LO > v {
            ((this + VAL_OFF) as *mut u32).write_unaligned(LO.to_bits());
        }
        let sel = lf_checker_rt::global::<u32>(SELECTOR).read_unaligned();
        let m = ((this + MODE_OFF) as *const u8).read();
        lf_checker_rt::global::<u8>(OUT_SLOT).write(m);
        if sel == WANT {
            lf_checker_rt::callee_cdecl!(CALLEE, u32, this + ARG_OFF);
        }
        1
    }
});
