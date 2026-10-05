// original: 0x00d282b0 target_score_by_distance
/// Score a target candidate from its distance and per-slot multipliers.
///
/// Measures the distance between the vec3 at `a1` and a reference vec3 (at
/// `[a2+0x20]+0x30`, or at `a2+0x10` when `[a2+0x20]` is null). A distance
/// below the range constant scores exactly 1.0 at once. Otherwise scans up
/// to 16 slots at `a0+0x18`, stopping at the first null: each live slot is
/// gate-checked through vtable slot `+0x2c` of `this` (with the slot and
/// `a2`), and passing slots multiply the accumulator — starting at the
/// image's 1.0 constant — by the float answer of vtable slot `+0x30`. When
/// the span constant exceeds the distance, the accumulator is blended with
/// `(1 - slope*distance)` as `(1-slope*d)*(1-acc) + acc`. Returns the score
/// in ST0. The float operation order is the original's.
///
/// The distance is also spilled into the incoming `a1`/`a2` stack slots; the
/// stack comparison is off for that reason (see narrowed).
///
/// Original: thiscall, ECX plus three stack words, float return in ST0.
lf_checker_rt::export!(thiscall, rw_00d282b0(this: u32, a0: u32, a1: u32, a2: u32) -> f32 {
    unsafe {
        const GATE: u32 = 1;
        const SCORE: u32 = 2;
        const ONE_VA: u32 = 0x00fe88e8;
        const RANGE_VA: u32 = 0x00fe8a94;
        const SPAN_VA: u32 = 0x00fe8b20;
        const SLOPE_VA: u32 = 0x00fe877c;
        const VT_GATE: u32 = 0x2c;
        const VT_SCORE: u32 = 0x30;
        let one = f32::from_bits(lf_checker_rt::global::<u32>(ONE_VA).read_unaligned());
        let base = ((a2 + 0x20) as *const u32).read_unaligned();
        let c: u32 = if base == 0 { a2 + 0x10 } else { base + 0x30 };
        let ax = f32::from_bits((a1 as *const u32).read_unaligned());
        let ay = f32::from_bits(((a1 + 4) as *const u32).read_unaligned());
        let az = f32::from_bits(((a1 + 8) as *const u32).read_unaligned());
        let cx = f32::from_bits((c as *const u32).read_unaligned());
        let cy = f32::from_bits(((c + 4) as *const u32).read_unaligned());
        let cz = f32::from_bits(((c + 8) as *const u32).read_unaligned());
        let dy = core::hint::black_box(ay) - core::hint::black_box(cy);
        let dx = core::hint::black_box(ax) - core::hint::black_box(cx);
        let dz = core::hint::black_box(az) - core::hint::black_box(cz);
        let dyy = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dxx = core::hint::black_box(dx) * core::hint::black_box(dx);
        let dzz = core::hint::black_box(dz) * core::hint::black_box(dz);
        let mut len2 = core::hint::black_box(dyy) + core::hint::black_box(dxx);
        len2 = core::hint::black_box(len2) + core::hint::black_box(dzz);
        let range =
            f32::from_bits(lf_checker_rt::global::<u32>(RANGE_VA).read_unaligned());
        let dist = core::hint::black_box(len2).sqrt();
        if range > dist {
            return 1.0;
        }
        let vt = (this as *const u32).read_unaligned();
        let gate_addr = ((vt + VT_GATE) as *const u32).read_unaligned();
        let score_addr = ((vt + VT_SCORE) as *const u32).read_unaligned();
        type Hook = extern "thiscall" fn(u32, u32, u32) -> u32;
        type HookF = extern "thiscall" fn(u32, u32, u32) -> f32;
        let gate: Hook = core::mem::transmute(gate_addr as usize);
        let score: HookF = core::mem::transmute(score_addr as usize);
        let mut acc = one;
        let mut i = 0u32;
        while i < 16 {
            let e = ((a0 + 0x18 + i * 4) as *const u32).read_unaligned();
            if e == 0 {
                break;
            }
            let g = gate(this, e, a2);
            if (g & 0xFF) != 0 {
                let s = score(this, e, a2);
                acc = core::hint::black_box(s) * core::hint::black_box(acc);
            }
            i += 1;
        }
        let span =
            f32::from_bits(lf_checker_rt::global::<u32>(SPAN_VA).read_unaligned());
        if span > dist {
            let slope =
                f32::from_bits(lf_checker_rt::global::<u32>(SLOPE_VA).read_unaligned());
            let k = core::hint::black_box(dist) * core::hint::black_box(slope);
            let t1 = core::hint::black_box(one) - core::hint::black_box(k);
            let t2 = core::hint::black_box(one) - core::hint::black_box(acc);
            let m = core::hint::black_box(t1) * core::hint::black_box(t2);
            acc = core::hint::black_box(m) + core::hint::black_box(acc);
        }
        acc
    }
});

/// Wrong version of rw_00d282b0: the blend drops the final `+ acc`.
lf_checker_rt::export!(thiscall, mut_00d282b0(this: u32, a0: u32, a1: u32, a2: u32) -> f32 {
    unsafe {
        const GATE: u32 = 1;
        const SCORE: u32 = 2;
        const ONE_VA: u32 = 0x00fe88e8;
        const RANGE_VA: u32 = 0x00fe8a94;
        const SPAN_VA: u32 = 0x00fe8b20;
        const SLOPE_VA: u32 = 0x00fe877c;
        const VT_GATE: u32 = 0x2c;
        const VT_SCORE: u32 = 0x30;
        let one = f32::from_bits(lf_checker_rt::global::<u32>(ONE_VA).read_unaligned());
        let base = ((a2 + 0x20) as *const u32).read_unaligned();
        let c: u32 = if base == 0 { a2 + 0x10 } else { base + 0x30 };
        let ax = f32::from_bits((a1 as *const u32).read_unaligned());
        let ay = f32::from_bits(((a1 + 4) as *const u32).read_unaligned());
        let az = f32::from_bits(((a1 + 8) as *const u32).read_unaligned());
        let cx = f32::from_bits((c as *const u32).read_unaligned());
        let cy = f32::from_bits(((c + 4) as *const u32).read_unaligned());
        let cz = f32::from_bits(((c + 8) as *const u32).read_unaligned());
        let dy = core::hint::black_box(ay) - core::hint::black_box(cy);
        let dx = core::hint::black_box(ax) - core::hint::black_box(cx);
        let dz = core::hint::black_box(az) - core::hint::black_box(cz);
        let dyy = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dxx = core::hint::black_box(dx) * core::hint::black_box(dx);
        let dzz = core::hint::black_box(dz) * core::hint::black_box(dz);
        let mut len2 = core::hint::black_box(dyy) + core::hint::black_box(dxx);
        len2 = core::hint::black_box(len2) + core::hint::black_box(dzz);
        let range =
            f32::from_bits(lf_checker_rt::global::<u32>(RANGE_VA).read_unaligned());
        let dist = core::hint::black_box(len2).sqrt();
        if range > dist {
            return 1.0;
        }
        let vt = (this as *const u32).read_unaligned();
        let gate_addr = ((vt + VT_GATE) as *const u32).read_unaligned();
        let score_addr = ((vt + VT_SCORE) as *const u32).read_unaligned();
        type Hook = extern "thiscall" fn(u32, u32, u32) -> u32;
        type HookF = extern "thiscall" fn(u32, u32, u32) -> f32;
        let gate: Hook = core::mem::transmute(gate_addr as usize);
        let score: HookF = core::mem::transmute(score_addr as usize);
        let mut acc = one;
        let mut i = 0u32;
        while i < 16 {
            let e = ((a0 + 0x18 + i * 4) as *const u32).read_unaligned();
            if e == 0 {
                break;
            }
            let g = gate(this, e, a2);
            if (g & 0xFF) != 0 {
                let s = score(this, e, a2);
                acc = core::hint::black_box(s) * core::hint::black_box(acc);
            }
            i += 1;
        }
        let span =
            f32::from_bits(lf_checker_rt::global::<u32>(SPAN_VA).read_unaligned());
        if span > dist {
            let slope =
                f32::from_bits(lf_checker_rt::global::<u32>(SLOPE_VA).read_unaligned());
            let k = core::hint::black_box(dist) * core::hint::black_box(slope);
            let t1 = core::hint::black_box(one) - core::hint::black_box(k);
            let t2 = core::hint::black_box(one) - core::hint::black_box(acc);
            let m = core::hint::black_box(t1) * core::hint::black_box(t2);
            // MUTANT: final `+ acc` dropped.
            acc = m;
        }
        acc
    }
});
