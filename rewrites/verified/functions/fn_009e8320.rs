// original: 0x009e8320 ground_height_radial_sweep (proposed)

/// Sweep vertical ray probes in a ring around a point (cdecl, eight stack
/// words), tracking the smallest and largest hit fraction.
///
/// `origin` points at three floats (the sweep centre `ox`, `oy`, `oz`).
/// `amplitude` scales the ring radius and `steps` sets the angular density:
/// the loop walks the angle `t` from `+0.0` up by `TAU/steps` while `TAU > t`
/// (`TAU` is read from a read-only image constant), so the body runs once per
/// step for positive finite `steps`. Each iteration takes the cosine and sine
/// of `t` (two callees that take XMM0 and return an `f32` in XMM0, scripted
/// here), scales both by `amplitude`, and casts the vertical segment from
/// `(ox + cos*r, oy - sin*r, oz)` down to `(same x, same y, oz - drop)`. The
/// cast callee (thiscall on a global world object, eight stack words: the
/// segment pointer, the hit-record pointer, then `0, 0x8e, -1, 7, 1, 0`)
/// answers nonzero on a hit and fills the hit record: the fraction at `+0x18`,
/// the four hit-point words at `+0x20`, and the hit entity word at `+0x48`.
/// The record is rebuilt every iteration from a global direction triple plus
/// fixed words (`0` at `+0x00/+0x40/+0x44/+0x48`, `0xffff` at `+0x4c`, zeroed
/// tail); only the fourth hit-point word carries over between iterations.
///
/// `min_frac`/`max_frac` start at large positive/negative sentinels and keep
/// the running minimum/maximum of hit fractions; a NaN fraction never updates
/// either (matching the original's `comiss` + `jbe` skip). `hit_point` (four
/// words) and `hit_entity` (one word) take the latest hit's values. Returns
/// 1 in AL if any probe hit, else 0; only AL is meaningful (the single caller
/// tests AL and the upper 24 bits of EAX are scratch residue).
///
/// Float operation order is the original's, pinned through `black_box`, so
/// results match bit for bit including NaN signs.
lf_checker_rt::export!(
    cdecl,
    rw_009e8320(
        origin: u32,
        min_frac: u32,
        max_frac: u32,
        amplitude: u32,
        steps: u32,
        drop: u32,
        hit_point: u32,
        hit_entity: u32,
    ) -> u32 {
        unsafe {
            const TAU_GLOB: u32 = 0x00fe8aec;
            const NEG_MASK_GLOB: u32 = 0x00fe8fa0;
            const DIR_GLOB: u32 = 0x01b4b320;
            const WORLD_GLOB: u32 = 0x012b9c78;
            const INIT_MIN: u32 = 0x497423f0;
            const INIT_MAX: u32 = 0xc97423f0;
            const COS: u32 = 1;
            const SIN: u32 = 2;
            const CAST: u32 = 3;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn wr32(a: u32, v: u32) {
                unsafe { (a as *mut u32).write_unaligned(v) }
            }
            #[inline(always)]
            fn mul(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) * core::hint::black_box(b)
            }
            #[inline(always)]
            fn add(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) + core::hint::black_box(b)
            }
            #[inline(always)]
            fn sub(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) - core::hint::black_box(b)
            }
            #[inline(always)]
            fn div(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) / core::hint::black_box(b)
            }

            wr32(min_frac, INIT_MIN);
            wr32(max_frac, INIT_MAX);
            let tau = f32::from_bits(rd32(lf_checker_rt::relocated(TAU_GLOB)));
            let amp = f32::from_bits(amplitude);
            let step = div(tau, f32::from_bits(steps));
            let drop_f = f32::from_bits(drop);
            let dir = lf_checker_rt::relocated(DIR_GLOB);
            let gx = rd32(dir);
            let gy = rd32(dir.wrapping_add(4));
            let gz = rd32(dir.wrapping_add(8));
            let neg_mask = rd32(lf_checker_rt::relocated(NEG_MASK_GLOB));
            let world = rd32(lf_checker_rt::relocated(WORLD_GLOB));
            let ox = f32::from_bits(rd32(origin));
            let oy = f32::from_bits(rd32(origin.wrapping_add(4)));
            let oz = f32::from_bits(rd32(origin.wrapping_add(8)));

            let mut t = 0.0f32;
            let mut hit_any = false;
            let mut rec = [0u32; 21];
            let mut seg = [0u32; 7];
            loop {
                rec[0] = 0;
                rec[4] = gx;
                rec[5] = gy;
                rec[6] = gz;
                rec[8] = gx;
                rec[9] = gy;
                rec[10] = gz;
                rec[12] = gx;
                rec[13] = gy;
                rec[14] = gz;
                rec[16] = 0;
                rec[17] = 0;
                rec[18] = 0;
                rec[19] = 0xffff;
                rec[20] = 0;
                let cos_out =
                    f32::from_bits(lf_checker_rt::callee_cdecl!(COS, u32, t.to_bits()));
                let cos_s = mul(cos_out, amp);
                let sin_out =
                    f32::from_bits(lf_checker_rt::callee_cdecl!(SIN, u32, t.to_bits()));
                let sin_s = mul(sin_out, amp);
                let sx = add(cos_s, ox);
                let neg_sin = f32::from_bits(sin_s.to_bits() ^ neg_mask);
                let sy = add(oy, neg_sin);
                let ez = sub(oz, drop_f);
                seg[0] = sx.to_bits();
                seg[1] = sy.to_bits();
                seg[2] = oz.to_bits();
                seg[3] = 0;
                seg[4] = sx.to_bits();
                seg[5] = sy.to_bits();
                seg[6] = ez.to_bits();
                let hit: u32 = lf_checker_rt::callee_thiscall!(
                    CAST,
                    u32,
                    world,
                    seg.as_mut_ptr() as u32,
                    rec.as_mut_ptr() as u32,
                    0,
                    0x8e,
                    0xffff_ffff,
                    7,
                    1,
                    0
                );
                if hit != 0 {
                    let frac = f32::from_bits(rec[6]);
                    if frac < f32::from_bits(rd32(min_frac)) {
                        wr32(min_frac, frac.to_bits());
                    }
                    if frac > f32::from_bits(rd32(max_frac)) {
                        wr32(max_frac, frac.to_bits());
                    }
                    wr32(hit_point, rec[8]);
                    wr32(hit_point.wrapping_add(4), rec[9]);
                    wr32(hit_point.wrapping_add(8), rec[10]);
                    wr32(hit_point.wrapping_add(12), rec[11]);
                    hit_any = true;
                    wr32(hit_entity, rec[18]);
                }
                t = add(t, step);
                if !(tau > t) {
                    break;
                }
            }
            hit_any as u32
        }
    }
);
