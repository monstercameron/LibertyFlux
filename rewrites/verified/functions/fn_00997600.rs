// original: 0x00997600 audio_mix_channel_frame (proposed)

/// Mix one frame of a channel's audio parameters into the output record.
///
/// `this` is the channel object (only address-derived sub-object pointers are
/// used, never dereferenced here); `outp` points to the output record whose
/// input gain is read at `+0x28` and whose mixed fields are written back at
/// `+0x04` through `+0x34` (floats at `+0x04`, `+0x0c`, `+0x10`, `+0x18`,
/// `+0x20`, `+0x24`, `+0x34`; truncated integers at `+0x08`, `+0x14`, `+0x1c`).
///
/// Behaviour: a scratch parameter block is initialised through callee 1 and
/// filled through callee 2 (the neighbouring audio function at file address
/// 0x997e00, stubbed here). The input gain is run through the shared
/// equalizer object (callee 3, file address 0x8acef0) seven times with
/// different sub-objects; four of those results are shaped by callee 4
/// (cdecl, file address 0x890030) and two are truncated to integers. Six
/// mixing calls (callee 5, file address 0x8ab800), each passed a zero
/// argument. The first mixing result overwrites the sixth equalizer
/// result's stack slot, so that equalizer result only feeds one truncated
/// integer. The final section scales the scratch block by
/// `1 - r0` and accumulates the mixing results scaled by `r0`, where `r0` is
/// the first equalizer result, adds the shared constants (1.0, 0.0, -100.0
/// from read-only data), and truncates three accumulators to integers. The
/// returned value is the last truncated integer.
///
/// All floating-point arithmetic uses the original's operand order (pinned
/// with `black_box`); truncation matches `cvttss2si` (NaN, infinity and
/// out-of-range values give `i32::MIN`).
///
/// Original: 0x00997600 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00997600(this: u32, outp: u32) -> u32 {
    unsafe {
        const GLOB_EQ: u32 = 0x01283898;
        const K_ONE: u32 = 0x00fe88e8;
        const K_ZERO: u32 = 0x00fe8628;
        const K_M100: u32 = 0x00fe8df8;
        const IN_GAIN: u32 = 0x28;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Truncate toward zero exactly like `cvttss2si`.
        #[inline(always)]
        fn trunc(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            let t = x; // truncation below via cast is exact once ranged
            if t >= 2147483648.0f32 || t < -2147483648.0f32 {
                i32::MIN
            } else {
                // In-range cast truncates toward zero, matching the instruction.
                // Rust emits `fisttp`/`cvttss2si` semantics for `as` here.
                t as i32
            }
        }
        #[inline(always)]
        unsafe fn eq(this_ptr: u32, bits: u32) -> f32 {
            unsafe { lf_checker_rt::callee_thiscall!(3, f32, this_ptr, bits) }
        }
        #[inline(always)]
        unsafe fn mix(this_ptr: u32, bits: u32) -> f32 {
            unsafe { lf_checker_rt::callee_thiscall!(5, f32, this_ptr, bits) }
        }

        let mut frame: [u32; 16] = [0; 16];
        let sptr = frame.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(1, u32, sptr);
        lf_checker_rt::callee_thiscall!(2, u32, this, sptr);

        let f0 = rdf(outp + IN_GAIN);
        let f0b = f0.to_bits();
        let r0: f32 = eq(lf_checker_rt::relocated(GLOB_EQ), f0b);
        let k0 = f32::from_bits(rd32(lf_checker_rt::relocated(K_ONE)));
        let scale = sub(k0, r0);
        let base = r0;

        let p1 = eq(this.wrapping_add(0x280), f0b);
        let q1: f32 = lf_checker_rt::callee_cdecl!(4, f32, p1.to_bits());
        let p2 = eq(this.wrapping_add(0x2a8), f0b);
        let q2: f32 = lf_checker_rt::callee_cdecl!(4, f32, p2.to_bits());
        let p3 = eq(this.wrapping_add(0x2f8), f0b);
        let q3: f32 = lf_checker_rt::callee_cdecl!(4, f32, p3.to_bits());
        let p4 = eq(this.wrapping_add(0x320), f0b);
        let q4: f32 = lf_checker_rt::callee_cdecl!(4, f32, p4.to_bits());
        let p5 = eq(this.wrapping_add(0x1b8), f0b);
        let ib = trunc(p5);
        let p6 = eq(this.wrapping_add(0x2d0), f0b);
        let id = trunc(p6);

        // Every mixing call takes a literal 0 argument; the sums below are
        // temporaries the original keeps on its stack for the final section.
        let u1: f32 = mix(this.wrapping_add(0x780), 0);
        let u2: f32 = mix(this.wrapping_add(0x5f0), 0);
        // a3 uses u1: the original stores u1 over p6's stack slot before
        // reading it here, so p6 only feeds the truncated integer below.
        let a3 = add(add(u1, u2), q1);
        let u3: f32 = mix(this.wrapping_add(0x6e0), 0);
        let u4: f32 = mix(this.wrapping_add(0x5f0), 0);
        let a5 = add(add(u3, u4), q2);
        let u5: f32 = mix(this.wrapping_add(0x730), 0);
        let a6 = add(u5, q3);
        let u6: f32 = mix(this.wrapping_add(0x640), 0);

        let k1 = f32::from_bits(rd32(lf_checker_rt::relocated(K_ZERO)));
        let k2 = f32::from_bits(rd32(lf_checker_rt::relocated(K_M100)));
        let s = |off: u32| rdf(sptr + off);
        let si = |off: u32| rd32(sptr + off) as i32;

        let g1 = mul(base, k1);
        let m2 = mul(a3, base);
        wrf(outp + 0x24, add(mul(s(0x24), scale), g1));
        let n1 = mul(add(u6, q4), base);
        wrf(outp + 0x0c, add(mul(s(0x0c), scale), m2));
        let m3 = mul(a5, base); // slot reused: original reads the stored a5 here, not u3
        wrf(outp + 0x04, add(mul(s(0x04), scale), m3));
        let m4 = mul(a6, base);
        wrf(outp + 0x20, add(mul(s(0x20), scale), m4));
        wrf(outp + 0x10, add(mul(s(0x10), scale), n1));
        let g2 = mul(base, k2);
        wrf(outp + 0x18, add(mul(s(0x18), scale), g2));
        wrf(outp + 0x34, add(mul(s(0x34), scale), g2));
        let c1 = mul(si(0x14) as f32, scale);
        let e1 = trunc(add(mul(si(0x1c) as f32, scale), g1));
        wr32(outp + 0x1c, e1 as u32);
        let e2 = trunc(add(c1, mul(ib as f32, base)));
        wr32(outp + 0x14, e2 as u32);
        let e3 = trunc(add(mul(si(0x08) as f32, scale), mul(id as f32, base)));
        wr32(outp + 0x08, e3 as u32);
        e3 as u32
    }
});
