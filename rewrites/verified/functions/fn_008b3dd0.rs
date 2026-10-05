// original: 0x008B3DD0 rage::audWaveshaperEffectPc::vf0 (merged symbol)

/// Configure the waveshaper's channel mask, then fill its 200-point curve.
///
/// `count` (argument 0) selects how many entries of the channel table at
/// file VA `0xe7d108` are tested against the mode byte at argument 1 `+0xe`:
/// `this+0x1e` counts the entries whose bit is set in the mode. `this+0x1d`
/// keeps the low byte of `count`, `this+0x1f` the mode byte. Then 200 curve
/// points are written at `this+0x24`, sweeping `x` from file VA `0xfe8d94`
/// up by file VA `0xfe870c` per point: when `this+0x18` is zero the point is
/// `x` itself, otherwise it is the shaper's answer (callee `0x8ace80`,
/// thiscall with `this+0x18` in `ecx` and five stack words
/// `(-1.0, 1.0, -1.0, 1.0, x)`, float result on the x87 stack). The callee
/// pops its five words: the original's post-call stack-relative reload of
/// `x` only lands correctly if it does. Returns 1 in `al`. Original is
/// thiscall with two stack words (the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_008B3DD0(this: u32, count: u32, desc: u32) -> u32 {
    const SHAPE: u32 = 1;
    const TABLE_FILE_VA: u32 = 0x00e7_d108;
    const X0_FILE_VA: u32 = 0x00fe_8d94;
    const STEP_FILE_VA: u32 = 0x00fe_870c;
    const COUNT_LO: u32 = 0x1d;
    const MATCHED: u32 = 0x1e;
    const MODE: u32 = 0x1f;
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
        ((this + COUNT_LO) as *mut u8).write(count as u8);
        let mode = ((desc + 0xe) as *const u8).read();
        ((this + MODE) as *mut u8).write(mode);
        ((this + MATCHED) as *mut u8).write(0);
        if count != 0 {
            let table = lf_checker_rt::relocated(TABLE_FILE_VA);
            let mut i = 0u32;
            while i < count {
                let bit = ((table + i * 4) as *const u32).read_unaligned();
                if mode as u32 & (1u32 << (bit & 31)) != 0 {
                    let m = (this + MATCHED) as *mut u8;
                    m.write(m.read().wrapping_add(1));
                }
                i += 1;
            }
        }
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
        1
    }
});
