// original: 0x00d1d870 seek_cover_slot_usable (proposed)

/// Tests whether a cover slot is usable from a source position, returning 1
/// for usable and 0 otherwise (thiscall, `this` in ECX, one stack word `src`,
/// low byte of EAX significant).
///
/// `src+0xd68` points at the slot object (a null pointer returns 0 at once);
/// its tag byte selects the direction source. When the tag's low three bits
/// equal 1, the slot query (id 1, thiscall on the slot with `(out, 0)`)
/// fills three words and the direction is that point minus the position
/// triple at `[src+0x20]+0x30..0x38`, normalised by `1/sqrt(len2)` (a zero
/// squared length scales by 0 instead of dividing). Otherwise the vector
/// callee (id 2, same shape) returns a pointer to the direction triple
/// directly. Either way the direction lands in `x`, `y`, `z`.
///
/// The direction is then combined with two scripted double answers (ids 3
/// and 4, converted to float) as `nx = x*d - y*c`, `ny = y*d + x*c`, and two
/// scripted float answers for the parameter at `this+0x30` (ids 5 and 6)
/// form the score `(-e*x)*nx + (f)*ny + 0`, computed in the original's exact
/// SSE order (the multiplications by zero keep NaN a NaN). The result is 1
/// when the score is ordered and above zero, else 0. No heap or global state
/// is written; everything lives in the frame.
///
/// Narrowings, all documented in the contract: ids 3 and 4 take their double
/// argument in XMM0 (always the read-only pi/2 constant), which the checker
/// cannot transport (it moves only the low dword), so that input is
/// uncompared; their scripted answers are constrained to doubles whose high
/// dword repeats the low dword, because the stub returns only the low dword
/// to the rewrite (the rewrite rebuilds the same 64 bits); the frame-pointer
/// arguments of ids 1 and 2 are skipped (their contents flow into the
/// compared result and calls).
///
/// Original: 0x00D1D870 (thiscall, one stack word, returns AL).
lf_checker_rt::export!(thiscall, rw_00d1d870(this: u32, src: u32) -> u32 {
    unsafe {
        const OFF_SLOT: u32 = 0xd68;
        const OFF_POS: u32 = 0x20;
        const OFF_PARAM: u32 = 0x30;
        const ONE_SCALE: u32 = 0xFE88E8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let node = rd32(src + OFF_SLOT);
        if node == 0 {
            return 0;
        }
        let tag = (node as *const u8).read() & 7;
        let (dx, dy, dz): (f32, f32, f32);
        if tag == 1 {
            let mut out = [0u32; 3];
            lf_checker_rt::callee_thiscall!(1, u32, node, out.as_mut_ptr() as u32, 0);
            let base = rd32(src + OFF_POS);
            let ox = f32::from_bits(out[0]);
            let oy = f32::from_bits(out[1]);
            let oz = f32::from_bits(out[2]);
            let px = sub(ox, f32::from_bits(rd32(base + 0x30)));
            let py = sub(oy, f32::from_bits(rd32(base + 0x34)));
            let pz = sub(oz, f32::from_bits(rd32(base + 0x38)));
            let n2 = add(add(mul(px, px), mul(py, py)), mul(pz, pz));
            // The original branches on ZF==PF after ucomiss against zero,
            // i.e. on ordered-equality: NaN takes the sqrt path like any
            // nonzero length.
            let scale = if n2 != 0.0 {
                div(
                    f32::from_bits(rd32(lf_checker_rt::relocated(ONE_SCALE))),
                    core::hint::black_box(n2).sqrt(),
                )
            } else {
                0.0
            };
            dx = mul(px, scale);
            dy = mul(py, scale);
            dz = mul(pz, scale);
        } else {
            let mut dummy = [0u32; 4];
            let p = lf_checker_rt::callee_thiscall!(2, u32, node, dummy.as_mut_ptr() as u32, 0);
            dx = f32::from_bits(rd32(p));
            dy = f32::from_bits(rd32(p + 4));
            dz = f32::from_bits(rd32(p + 8));
        }
        // Scripted double answers; the stub returns only the low dword, so
        // the contract constrains them to hi == lo and the rewrite rebuilds
        // the same 64 bits the original converts.
        let c_lo = lf_checker_rt::callee_cdecl!(3, u32,);
        let d_lo = lf_checker_rt::callee_cdecl!(4, u32,);
        let c = f64::from_bits((c_lo as u64) | ((c_lo as u64) << 32)) as f32;
        let d = f64::from_bits((d_lo as u64) | ((d_lo as u64) << 32)) as f32;
        let nx = sub(mul(dx, d), mul(dy, c));
        let ny = add(mul(dy, d), mul(dx, c));
        // Scripted float answers for the parameter, passed in XMM0 on the
        // original side (the stub's transport carries the rewrite's copy).
        let param = rd32(this + OFF_PARAM);
        let e = f32::from_bits(lf_checker_rt::callee_stdcall!(5, u32, param));
        let f = f32::from_bits(lf_checker_rt::callee_stdcall!(6, u32, param));
        let zero = 0.0f32;
        let t5 = sub(mul(f, zero), e);
        let t8 = add(mul(e, zero), f);
        let score = add(add(mul(t8, ny), mul(t5, nx)), mul(dz, zero));
        u32::from(score > 0.0)
    }
});
