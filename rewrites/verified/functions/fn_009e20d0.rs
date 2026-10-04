// original: 0x009e20d0 audio_angle_bounds_update
/// Update direction-angle bounds for four tracked portals (thiscall).
///
/// `obj` is the tracker object: bytes `0x54..0x74` hold running min/max
/// angle bounds, bytes `0x80..0xBC` a coefficient matrix. `vecs` points at
/// four 16-byte direction records. For each record the rewrite forms three
/// weighted sums, divides by the third (with 1.0), and passes the two ratios
/// through a scripted angle helper; each answer is negated and, when the
/// divisor is positive, wrapped by pi toward the ratio's sign. The two
/// angles tighten the running bounds, which are then clamped against stored
/// limits, and a scripted range call may reset one bound pair. Returns the
/// range call's answer, or the stored limit when the reset fires.
export!(thiscall, rw_9e20d0(obj: *mut u8, vecs: *const u8) -> u32 {
    #[inline(always)]
    unsafe fn rf(base: *const u8, off: usize) -> f32 {
        *(base.add(off) as *const f32)
    }
    #[inline(always)]
    unsafe fn wf(base: *mut u8, off: usize, v: f32) {
        *(base.add(off) as *mut f32) = v;
    }
    #[inline(always)]
    unsafe fn ri(base: *const u8, off: usize) -> u32 {
        *(base.add(off) as *const u32)
    }
    #[inline(always)]
    unsafe fn wi(base: *mut u8, off: usize, v: u32) {
        *(base.add(off) as *mut u32) = v;
    }
    // Scalar SSE ops with exact NaN-payload propagation. When both operands
    // are quiet NaNs the destination payload wins, but rustc may emit either
    // operand order, so plain `+`/`*` can differ from the original in the
    // payload while agreeing on every non-NaN value. Selecting a NaN operand
    // explicitly keeps the exact bits; non-NaN operands go through the real
    // instruction, which is order-insensitive there (round-to-nearest, and
    // 0/0-style cases produce the indefinite NaN either way around).
    // Signaling NaNs cannot occur: inputs are quieted and SSE ops never
    // produce them.
    #[inline(always)]
    fn fadd(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            dest
        } else if src.is_nan() {
            src
        } else {
            dest + src
        }
    }
    #[inline(always)]
    fn fsub(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            dest
        } else if src.is_nan() {
            src
        } else {
            dest - src
        }
    }
    #[inline(always)]
    fn fmul(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            dest
        } else if src.is_nan() {
            src
        } else {
            dest * src
        }
    }
    #[inline(always)]
    fn fdiv(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            dest
        } else if src.is_nan() {
            src
        } else {
            dest / src
        }
    }
    unsafe {
        let sign: u32 = *global::<u32>(0xFE8FA0);
        let pi: f32 = *global::<f32>(0xFE8AA0);
        let one: f32 = *global::<f32>(0xFE88E8);
        let mut rec = vecs.add(8);
        let mut count: u32 = 4;
        let mut last: u32 = 0;
        loop {
            let v0 = rf(rec.sub(8), 0);
            let v1 = rf(rec.sub(4), 0);
            let v2 = rf(rec, 0);
            let mut a = fmul(rf(obj, 0x80), v0);
            a = fadd(a, fmul(v1, rf(obj, 0x90)));
            a = fadd(a, fmul(v2, rf(obj, 0xA0)));
            a = fadd(a, rf(obj, 0xB0));
            let mut b = fmul(rf(obj, 0x94), v1);
            b = fadd(b, fmul(v0, rf(obj, 0x84)));
            b = fadd(b, fmul(rf(obj, 0xA4), v2));
            b = fadd(b, rf(obj, 0xB4));
            let mut c = fmul(rf(obj, 0x98), v1);
            c = fadd(c, fmul(rf(obj, 0x88), v0));
            c = fadd(c, fmul(rf(obj, 0xA8), v2));
            c = fadd(c, rf(obj, 0xB8));
            let inv = fdiv(one, c);
            let r0: u32 = callee_cdecl!(1, u32, fmul(inv, a).to_bits());
            let mut n0 = f32::from_bits(r0 ^ sign);
            if c > 0.0 {
                if a > 0.0 {
                    n0 = fadd(n0, pi);
                } else {
                    n0 = fsub(n0, pi);
                }
            }
            let r1: u32 = callee_cdecl!(1, u32, fmul(inv, b).to_bits());
            let mut n1 = f32::from_bits(r1 ^ sign);
            if c > 0.0 {
                if b > 0.0 {
                    n1 = fadd(n1, pi);
                } else {
                    n1 = fsub(n1, pi);
                }
            }
            if rf(obj, 0x5C) > n0 {
                wf(obj, 0x5C, n0);
            }
            if n0 > rf(obj, 0x60) {
                wf(obj, 0x60, n0);
            }
            if rf(obj, 0x54) > n1 {
                wf(obj, 0x54, n1);
            }
            if n1 > rf(obj, 0x58) {
                wf(obj, 0x58, n1);
            }
            if rf(obj, 0x6C) > rf(obj, 0x5C) {
                wf(obj, 0x5C, rf(obj, 0x6C));
            }
            if rf(obj, 0x60) > rf(obj, 0x70) {
                wf(obj, 0x60, rf(obj, 0x70));
            }
            if rf(obj, 0x64) > rf(obj, 0x54) {
                wf(obj, 0x54, rf(obj, 0x64));
            }
            if rf(obj, 0x58) > rf(obj, 0x68) {
                wf(obj, 0x58, rf(obj, 0x68));
            }
            let mut local: f32 = f32::from_bits(0x447A0000);
            last = callee_thiscall!(
                2,
                u32,
                vecs as u32,
                obj.add(0x20) as u32,
                &mut local as *mut f32 as u32
            );
            if one > local {
                wi(obj, 0x5C, ri(obj, 0x6C));
                wi(obj, 0x60, ri(obj, 0x70));
                last = ri(obj, 0x70);
            }
            rec = rec.add(0x10);
            count -= 1;
            *obj.add(0x74) = 1;
            if count == 0 {
                break;
            }
        }
        last
    }
});
