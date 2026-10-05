// original: 0x008B3E90 audio_curve_fill (proposed)

/// Fill the 200-point shaper curve at `this+0x24` (shared tail of vf0).
///
/// Sweeps `x` from file VA `0xfe8d94` up by file VA `0xfe870c` per point:
/// when `this+0x18` is zero the point is `x` itself, otherwise it is the
/// shaper's answer (callee `0x8ace80`, thiscall with `this+0x18` in `ecx`
/// and five stack words `(-1.0, 1.0, -1.0, 1.0, x)`, float result on the
/// x87 stack, five words popped). No return value. Original is thiscall with
/// no stack words (plain `ret`; it preserves `ecx` across the call).
lf_checker_rt::export!(thiscall, rw_008B3E90(this: u32) -> u32 {
    const SHAPE: u32 = 1;
    const X0_FILE_VA: u32 = 0x00fe_8d94;
    const STEP_FILE_VA: u32 = 0x00fe_870c;
    const COND: u32 = 0x18;
    const CURVE: u32 = 0x24;
    const POINTS: u32 = 200;
    const NEG_ONE: u32 = 0xbf80_0000;
    const POS_ONE: u32 = 0x3f80_0000;
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    unsafe {
        let mut x = f32::from_bits(
            (lf_checker_rt::relocated(X0_FILE_VA) as *const u32).read_unaligned());
        let step = f32::from_bits(
            (lf_checker_rt::relocated(STEP_FILE_VA) as *const u32).read_unaligned());
        let cond = ((this + COND) as *const u32).read_unaligned();
        let mut p = this + CURVE;
        let mut n = POINTS;
        while n != 0 {
            if cond == 0 {
                ((p) as *mut u32).write_unaligned(x.to_bits());
            } else {
                let r: f32 = lf_checker_rt::callee_thiscall!(
                    SHAPE, f32, cond, NEG_ONE, POS_ONE, NEG_ONE, POS_ONE, x.to_bits());
                (p as *mut u32).write_unaligned(r.to_bits());
            }
            x = add(x, step);
            p += 4;
            n -= 1;
        }
    }
    0
});
