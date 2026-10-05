// original: 0x00CD0220 melee_range_check (proposed)

/// Decide whether a melee attack can start, and report it through an out flag.
///
/// `this` is the melee task object (flag byte at `+TASK_FLAGS`, object id at
/// `+TASK_ID`, state bytes at `+TASK_STATE0/1`, dwords at `+TASK_D0/D1`);
/// `a1` is a ped object (flag bytes at `+PED_F0/F1`, float-array link at
/// `+PED_VEC`); `a2` is an opaque value forwarded as `this` to the query
/// callees; `a3` points to one dword consumed by the resolver; `a4` is the
/// out flag, cleared on entry and set to 1 on success. Returns 1 with the
/// flag set when the attack is accepted, else 0 (thiscall, AL is the result).
///
/// Behaviour: after a gate query and clearing the flag, a set bit 1 in the
/// task flags selects the fast path (a six-argument validation call, then a
/// resolve-and-compare sequence that succeeds unless the resolved rank is
/// below the queried one). Otherwise the ped flags are consulted: one
/// combination tries the same validation call, another combination fails at
/// once. The slow path then checks task state, optionally runs a state
/// validator, and either takes a constant score or computes one from the
/// distance between the ped position and a queried point: root-sum-square
/// minus an offset, normalised, clamped to [0, 1] with ordered comparisons
/// (NaN stays NaN), passed with a curve parameter to a shaping call, scaled
/// and clamped again. The score beats a count-scaled threshold to reach the
/// final validation call, which decides success.
///
/// Original: 0x00CD0220 (thiscall, four stack words; returns AL with
/// nonzero upper bytes left over, so the contract compares AL only).
lf_checker_rt::export!(thiscall, rw_00CD0220(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    // Final validation call shared by the slow path.
    unsafe fn finish(a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
        unsafe {
            let ok: u32 = lf_checker_rt::callee_thiscall!(
                12, u32, a2, a1, a3, 0u32, 0u32, 0u32, 0u32
            );
            if (ok as u8) != 0 {
                ((a4) as *mut u8).write(1);
                return 1;
            }
            0
        }
    }

    // Distance-based score: root-sum-square of the ped position minus the
    // queried point, normalised by (K_NORM_HI - K_NORM_LO), clamped to
    // [0, K_CLAMP_HI], shaped through the curve call, scaled by K_SCALE and
    // clamped to [K_LO, K_SCALE]. All comparisons are ordered (a NaN keeps
    // its value), matching the original's comiss/jbe pairs.
    unsafe fn score(this: u32, a1: u32) -> f32 {
        unsafe {
            const K_NORM_HI: u32 = 0x10519a0;
            const K_NORM_LO: u32 = 0x105199c;
            const K_CLAMP_HI: u32 = 0xfe88e8;
            const K_CURVE: u32 = 0x10519a4;
            const K_SCALE: u32 = 0x10519ac;
            const K_LO: u32 = 0x10519a8;
            #[inline(always)]
            fn sub(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) - core::hint::black_box(b)
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
            unsafe fn kf(va: u32) -> f32 {
                unsafe {
                    f32::from_bits(
                        (lf_checker_rt::relocated(va) as *const u32).read_unaligned(),
                    )
                }
            }
            // The callee fills three floats at the passed pointer (the lea
            // runs after the first push, so the pointer aims at the first
            // word the original later reads back).
            let mut buf = [0u32; 3];
            let pid = ((this + 0x50) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(9, u32, pid, buf.as_mut_ptr() as u32);
            let base = ((a1 + 0x20) as *const u32).read_unaligned();
            let dx = sub(
                f32::from_bits(((base + 0x30) as *const u32).read_unaligned()),
                f32::from_bits(buf[0]),
            );
            let dy = sub(
                f32::from_bits(((base + 0x34) as *const u32).read_unaligned()),
                f32::from_bits(buf[1]),
            );
            let dz = sub(
                f32::from_bits(((base + 0x38) as *const u32).read_unaligned()),
                f32::from_bits(buf[2]),
            );
            let mut r = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            r = core::hint::black_box(r).sqrt();
            let klo = kf(K_NORM_LO);
            r = sub(r, klo);
            r = core::hint::black_box(r) / core::hint::black_box(sub(kf(K_NORM_HI), klo));
            if r < 0.0 {
                r = 0.0;
            }
            let khi = kf(K_CLAMP_HI);
            if r > khi {
                r = khi;
            }
            lf_checker_rt::callee_cdecl!(10, u32, sub(khi, r).to_bits(), kf(K_CURVE).to_bits());
            let scale = kf(K_SCALE);
            let t = mul(sub(khi, r), scale);
            let mut x = t;
            if kf(K_LO) > t {
                x = kf(K_LO);
            }
            if t > scale {
                x = scale;
            }
            x
        }
    }

    unsafe {
        const TASK_FLAGS: u32 = 0xf1;
        const TASK_D0: u32 = 0xcc;
        const TASK_D1: u32 = 0xd0;
        const TASK_STATE0: u32 = 0x4c;
        const TASK_STATE1: u32 = 0x50;
        const PED_F0: u32 = 0x218;
        const PED_F1: u32 = 0x219;
        const K_CONST: u32 = 0x10519b0;
        const K_STEP: u32 = 0xfe8684;
        const RESOLVER_OBJ: u32 = 0x171c968;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let gate: u32 = lf_checker_rt::callee_thiscall!(1, u32, a2);
        wr8(a4, 0);
        let f1 = rd8(this + TASK_FLAGS);
        if (f1 >> 1) & 1 != 0 {
            // Fast path: validate, then resolve and compare ranks.
            let ok: u32 = lf_checker_rt::callee_thiscall!(
                2, u32, a2, a1, a3, 1u32,
                rd32(this + TASK_D0), rd32(this + TASK_D1), u32::from(f1 & 1)
            );
            if (ok as u8) != 0 {
                let inner: u32 = lf_checker_rt::callee_thiscall!(
                    3, u32, lf_checker_rt::relocated(RESOLVER_OBJ), rd32(a3)
                );
                if (gate as u8) == 0 {
                    wr8(a4, 1);
                    return 1;
                }
                if inner != 0 {
                    let rank: u32 = lf_checker_rt::callee_thiscall!(4, u32, inner);
                    let limit: u32 = lf_checker_rt::callee_thiscall!(5, u32, a2);
                    if rank >= limit {
                        wr8(a4, 1);
                        return 1;
                    }
                }
            }
        }
        let b0 = rd8(a1 + PED_F0);
        let b1 = rd8(a1 + PED_F1);
        if b0 == 0 && b1 != 0 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(
                6, u32, a2, a1, a3, 0u32, 0u32, 0u32, 0u32
            );
            if (ok as u8) != 0 {
                wr8(a4, 1);
                return 1;
            }
            return 0;
        }
        if b1 != 0 {
            return 0;
        }
        if rd32(this + TASK_STATE1) == 0 {
            return 0;
        }
        if rd8(this + TASK_STATE0) != 0 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(7, u32, this + 0x44);
            if (ok as u8) != 0 {
                return finish(a1, a2, a3, a4);
            }
        }
        let t: u32 = lf_checker_rt::callee_thiscall!(8, u32, a2);
        let x = if (t as u8) != 0 {
            f32::from_bits(rd32(lf_checker_rt::relocated(K_CONST)))
        } else {
            score(this, a1)
        };
        let n: u32 = lf_checker_rt::callee_cdecl!(11, u32,);
        let f = core::hint::black_box((n as i32) as f32)
            * core::hint::black_box(f32::from_bits(rd32(lf_checker_rt::relocated(K_STEP))));
        if !(x > f) {
            return 0;
        }
        finish(a1, a2, a3, a4)
    }
});
