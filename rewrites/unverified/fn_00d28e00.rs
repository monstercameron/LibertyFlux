// original: 0x00d28e00 target_direction_solve
/// Solve a target direction vector into the 16-byte record at `a2`.
///
/// Zeroes the record, then asks a finder call for a candidate: a null answer
/// returns 0 with the record zeroed (plus a scratch word, see below). From a
/// live candidate reads a weight `(range - [cand+0x30]) * gain` from image
/// constants; a non-positive weight returns the candidate with the record
/// still zeroed. Otherwise measures the 2D distance between the vec at `a0`
/// and a reference (`[a1+0x20]+0x30`, or `a1+0x10` when null): below the
/// near constant the spill is 0.0, above the far constant it keeps the
/// weight, between them it is `(1 - (far-dist)*ramp) * weight`. Two
/// single-float helper calls on `[cand+0x10]` then combine with the spill —
/// through a multiply-by-zero, subtract, sign-flip, add, sign-flip, multiply
/// sequence in the original's exact order — into the record's first three
/// words. Returns the second helper answer word on the main path, the candidate (or 0) on the early exits.
///
/// Two record words come from uninitialized frame scratch (stack_fill 0, the
/// rewrite writes 0). The helpers take their float in XMM0, carried on the
/// rewrite side by a phantom stack word (see narrowed).
///
/// Original: stdcall, three stack words.
lf_checker_rt::export!(stdcall, rw_00d28e00(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FIND: u32 = 1;
        const TRIG_A: u32 = 2;
        const TRIG_B: u32 = 3;
        const RANGE_VA: u32 = 0x00fe8a94;
        const GAIN_VA: u32 = 0x00fe8808;
        const NEAR_VA: u32 = 0x00fe8ad8;
        const FAR_VA: u32 = 0x00fe8b20;
        const ONE_VA: u32 = 0x00fe88e8;
        const RAMP_VA: u32 = 0x00fe879c;
        const OUT_VA: u32 = 0x00fe8d1c;
        const SIGNMASK_VA: u32 = 0x00fe8fa0;
        (a2 as *mut u32).write_unaligned(0);
        ((a2 + 4) as *mut u32).write_unaligned(0);
        ((a2 + 8) as *mut u32).write_unaligned(0);
        ((a2 + 0xc) as *mut u32).write_unaligned(0);
        let found: u32 = lf_checker_rt::callee_cdecl!(FIND, u32,);
        if found == 0 {
            return 0;
        }
        let range = f32::from_bits(lf_checker_rt::global::<u32>(RANGE_VA).read_unaligned());
        let c30 = f32::from_bits(((found + 0x30) as *const u32).read_unaligned());
        let mut w = core::hint::black_box(range) - core::hint::black_box(c30);
        let gain = f32::from_bits(lf_checker_rt::global::<u32>(GAIN_VA).read_unaligned());
        w = core::hint::black_box(w) * core::hint::black_box(gain);
        if !(w > 0.0) {
            return found;
        }
        let base = ((a1 + 0x20) as *const u32).read_unaligned();
        let r: u32 = if base == 0 { a1 + 0x10 } else { base + 0x30 };
        let ax = f32::from_bits((a0 as *const u32).read_unaligned());
        let ay = f32::from_bits(((a0 + 4) as *const u32).read_unaligned());
        let rx = f32::from_bits((r as *const u32).read_unaligned());
        let ry = f32::from_bits(((r + 4) as *const u32).read_unaligned());
        let dy = core::hint::black_box(ay) - core::hint::black_box(ry);
        let dx = core::hint::black_box(ax) - core::hint::black_box(rx);
        let dyy = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dxx = core::hint::black_box(dx) * core::hint::black_box(dx);
        let len2 = core::hint::black_box(dyy) + core::hint::black_box(dxx);
        let dist = core::hint::black_box(len2).sqrt();
        let near = f32::from_bits(lf_checker_rt::global::<u32>(NEAR_VA).read_unaligned());
        let mut spill = w;
        if near > dist {
            spill = 0.0;
        } else {
            let far = f32::from_bits(lf_checker_rt::global::<u32>(FAR_VA).read_unaligned());
            if far > dist {
                let one =
                    f32::from_bits(lf_checker_rt::global::<u32>(ONE_VA).read_unaligned());
                let t = core::hint::black_box(far) - core::hint::black_box(dist);
                let ramp =
                    f32::from_bits(lf_checker_rt::global::<u32>(RAMP_VA).read_unaligned());
                let u = core::hint::black_box(t) * core::hint::black_box(ramp);
                let v = core::hint::black_box(one) - core::hint::black_box(u);
                spill = core::hint::black_box(v) * core::hint::black_box(w);
            }
        }
        let f0 = f32::from_bits(((found + 0x10) as *const u32).read_unaligned());
        let ra: u32 = lf_checker_rt::callee_thiscall!(TRIG_A, u32, found, f0.to_bits());
        let fa = f32::from_bits(ra);
        let rb: u32 = lf_checker_rt::callee_stdcall!(TRIG_B, u32, f0.to_bits());
        let fb = f32::from_bits(rb);
        let zero = 0.0f32;
        let mut p = core::hint::black_box(fb) * core::hint::black_box(zero);
        p = core::hint::black_box(p) - core::hint::black_box(fa);
        let mut q = core::hint::black_box(fa) * core::hint::black_box(zero);
        let mask = lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
        p = f32::from_bits(p.to_bits() ^ mask);
        q = core::hint::black_box(q) + core::hint::black_box(fb);
        let o1 = core::hint::black_box(p) * core::hint::black_box(spill);
        let outk = f32::from_bits(lf_checker_rt::global::<u32>(OUT_VA).read_unaligned());
        let s = core::hint::black_box(spill) * core::hint::black_box(outk);
        q = f32::from_bits(q.to_bits() ^ mask);
        let o0 = core::hint::black_box(spill) * core::hint::black_box(q);
        (a2 as *mut u32).write_unaligned(o1.to_bits());
        ((a2 + 8) as *mut u32).write_unaligned(s.to_bits());
        ((a2 + 4) as *mut u32).write_unaligned(o0.to_bits());
        ((a2 + 0xc) as *mut u32).write_unaligned(0);
        rb
    }
});

/// Wrong version of rw_00d28e00: the third word stores `o0`, not the spill.
lf_checker_rt::export!(stdcall, mut_00d28e00(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FIND: u32 = 1;
        const TRIG_A: u32 = 2;
        const TRIG_B: u32 = 3;
        const RANGE_VA: u32 = 0x00fe8a94;
        const GAIN_VA: u32 = 0x00fe8808;
        const NEAR_VA: u32 = 0x00fe8ad8;
        const FAR_VA: u32 = 0x00fe8b20;
        const ONE_VA: u32 = 0x00fe88e8;
        const RAMP_VA: u32 = 0x00fe879c;
        const OUT_VA: u32 = 0x00fe8d1c;
        const SIGNMASK_VA: u32 = 0x00fe8fa0;
        (a2 as *mut u32).write_unaligned(0);
        ((a2 + 4) as *mut u32).write_unaligned(0);
        ((a2 + 8) as *mut u32).write_unaligned(0);
        ((a2 + 0xc) as *mut u32).write_unaligned(0);
        let found: u32 = lf_checker_rt::callee_cdecl!(FIND, u32,);
        if found == 0 {
            return 0;
        }
        let range = f32::from_bits(lf_checker_rt::global::<u32>(RANGE_VA).read_unaligned());
        let c30 = f32::from_bits(((found + 0x30) as *const u32).read_unaligned());
        let mut w = core::hint::black_box(range) - core::hint::black_box(c30);
        let gain = f32::from_bits(lf_checker_rt::global::<u32>(GAIN_VA).read_unaligned());
        w = core::hint::black_box(w) * core::hint::black_box(gain);
        if !(w > 0.0) {
            return found;
        }
        let base = ((a1 + 0x20) as *const u32).read_unaligned();
        let r: u32 = if base == 0 { a1 + 0x10 } else { base + 0x30 };
        let ax = f32::from_bits((a0 as *const u32).read_unaligned());
        let ay = f32::from_bits(((a0 + 4) as *const u32).read_unaligned());
        let rx = f32::from_bits((r as *const u32).read_unaligned());
        let ry = f32::from_bits(((r + 4) as *const u32).read_unaligned());
        let dy = core::hint::black_box(ay) - core::hint::black_box(ry);
        let dx = core::hint::black_box(ax) - core::hint::black_box(rx);
        let dyy = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dxx = core::hint::black_box(dx) * core::hint::black_box(dx);
        let len2 = core::hint::black_box(dyy) + core::hint::black_box(dxx);
        let dist = core::hint::black_box(len2).sqrt();
        let near = f32::from_bits(lf_checker_rt::global::<u32>(NEAR_VA).read_unaligned());
        let mut spill = w;
        if near > dist {
            spill = 0.0;
        } else {
            let far = f32::from_bits(lf_checker_rt::global::<u32>(FAR_VA).read_unaligned());
            if far > dist {
                let one =
                    f32::from_bits(lf_checker_rt::global::<u32>(ONE_VA).read_unaligned());
                let t = core::hint::black_box(far) - core::hint::black_box(dist);
                let ramp =
                    f32::from_bits(lf_checker_rt::global::<u32>(RAMP_VA).read_unaligned());
                let u = core::hint::black_box(t) * core::hint::black_box(ramp);
                let v = core::hint::black_box(one) - core::hint::black_box(u);
                spill = core::hint::black_box(v) * core::hint::black_box(w);
            }
        }
        let f0 = f32::from_bits(((found + 0x10) as *const u32).read_unaligned());
        let ra: u32 = lf_checker_rt::callee_thiscall!(TRIG_A, u32, found, f0.to_bits());
        let fa = f32::from_bits(ra);
        let rb: u32 = lf_checker_rt::callee_stdcall!(TRIG_B, u32, f0.to_bits());
        let fb = f32::from_bits(rb);
        let zero = 0.0f32;
        let mut p = core::hint::black_box(fb) * core::hint::black_box(zero);
        p = core::hint::black_box(p) - core::hint::black_box(fa);
        let mut q = core::hint::black_box(fa) * core::hint::black_box(zero);
        let mask = lf_checker_rt::global::<u32>(SIGNMASK_VA).read_unaligned();
        p = f32::from_bits(p.to_bits() ^ mask);
        q = core::hint::black_box(q) + core::hint::black_box(fb);
        let o1 = core::hint::black_box(p) * core::hint::black_box(spill);
        let outk = f32::from_bits(lf_checker_rt::global::<u32>(OUT_VA).read_unaligned());
        let s = core::hint::black_box(spill) * core::hint::black_box(outk);
        q = f32::from_bits(q.to_bits() ^ mask);
        let o0 = core::hint::black_box(spill) * core::hint::black_box(q);
        (a2 as *mut u32).write_unaligned(o1.to_bits());
        // MUTANT: o0 instead of s in the third word.
        ((a2 + 8) as *mut u32).write_unaligned(o0.to_bits());
        ((a2 + 4) as *mut u32).write_unaligned(o0.to_bits());
        ((a2 + 0xc) as *mut u32).write_unaligned(0);
        rb
    }
});
