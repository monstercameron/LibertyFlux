// original: 0x0061BD70 net_channel_state_update (proposed)
//
// fastcall(obj, src, a0, a1, a2): copy twelve floats out of the 64-byte
// record at `a2` into a frame buffer (four groups of three, skipping the
// pad word of each 16-byte group), run the buffer through two validation
// helpers, and on success update `obj` from `src`.
//
// `obj` (ECX) is updated at `obj + 0x4be0 + {0x2b8,0x2bc,0x2c0,0x2c4}`
// with the flag byte at `+0x2f0` set; `src` (EDX) supplies the inputs at
// `+0x2a0..0x2c4`. The blended float is `src.0x2bc` when it is NaN,
// otherwise the image constant (1.0) when `src.0x2a4` is ordered,
// otherwise `(float(src.0x2b0) * src.0x2a0) / (float(src.0x2b4) *
// src.0x2a4)` in the original's operand order. Two scaled integers
// derived from `src` fields go to the sixth callee, six words from
// `a0`/`a1` are copied to `obj + 0x5520..0x5538`, and the word at `obj +
// 0x4000` is cleared. The return value is incidental (the last callee's
// answer, or the validator's full EAX on early exit).
//
// Original: 0x0061BD70 (fastcall argument arrival, ECX, EDX, three stack
// words, but a plain ret: caller cleanup, which Rust cannot express, so
// the contract leaves the esp check off).
unsafe fn fn2_inner(obj: u32, src: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const OUT_BIAS: u32 = 0x4be0;
        const SCR_BIAS: u32 = 0x47f0;
        const FCONST_VA: u32 = 0x00fe88e8;
        const C_COPY: u32 = 1;
        const C_VALID: u32 = 2;
        const C_APPLY: u32 = 3;
        const C_TICK: u32 = 4;
        const C_SCALE: u32 = 5;
        const C_COMMIT: u32 = 6;
        const C_DONE: u32 = 7;

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        // Exact cvttss2si: truncate toward zero; NaN, infinities and
        // out-of-range values yield 0x80000000 (Rust `as` saturates
        // instead, so the edges are handled explicitly).
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            let x = core::hint::black_box(x);
            if x.is_nan() || x >= 2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }

        // Zeroed (not MaybeUninit): the original leaves the buffer pads and
        // the scratch word as unwritten stack, which reads as zero only
        // under the contract's zero stack fill; the rewrite writes the zero
        // explicitly (r-b184 precedent), disclosed in the contract record.
        let mut buf = [0u32; 16];
        let b = buf.as_mut_ptr() as *mut u32;
        let mut gi = 0usize;
        while gi < 4 {
            let s = (a2 + gi as u32 * 16) as *const u32;
            let d = b.add(gi * 4);
            d.write_unaligned(s.read_unaligned());
            d.add(1).write_unaligned(s.add(1).read_unaligned());
            d.add(2).write_unaligned(s.add(2).read_unaligned());
            gi += 1;
        }
        let mut scratch = core::mem::MaybeUninit::<[u8; 96]>::uninit();
        let sc = scratch.as_mut_ptr() as *mut u8;
        // The snapped scratch word is unwritten stack (zero under the fill)
        // on the original side; write the zero explicitly here.
        (sc.add(0x40) as *mut u32).write_unaligned(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_COPY, u32, sc.add(0x40) as u32, a2);
        let valid: u32 = lf_checker_rt::callee_thiscall!(C_VALID, u32, sc as u32, b as u32);
        if valid & 0xff == 0 {
            return valid;
        }
        // Second copy call re-passes the buffer head and the fill word.
        let _: u32 = lf_checker_rt::callee_cdecl!(C_COPY, u32, b as u32, sc.add(0x40) as u32);
        // Apply call: `this` is the object's first word (a value, compared
        // directly), arg1 the buffer itself (address skipped, head snapped).
        let _: u32 =
            lf_checker_rt::callee_thiscall!(C_APPLY, u32, obj, 1, b as u32, 1);
        let f_bc = rdf(src + 0x2bc);
        let blended = if f_bc == 0.0 {
            let f_a4 = rdf(src + 0x2a4);
            if f_a4 == 0.0 {
                f32::from_bits(rd32(lf_checker_rt::relocated(FCONST_VA)))
            } else {
                let p = mul((rd32(src + 0x2b0) as i32) as f32, rdf(src + 0x2a0));
                let q = mul((rd32(src + 0x2b4) as i32) as f32, f_a4);
                div(p, q)
            }
        } else {
            f_bc
        };
        let w = rd32(src + 0x2b8);
        let out = obj.wrapping_add(OUT_BIAS);
        wr32(out + 0x2b8, w);
        wrf(out + 0x2bc, blended);
        wrf(out + 0x2c0, rdf(src + 0x2c0));
        wrf(out + 0x2c4, rdf(src + 0x2c4));
        ((out + 0x2f0) as *mut u8).write(1);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_TICK, u32,);
        let f_c4 = rdf(src + 0x2c4);
        let f_c0 = rdf(src + 0x2c0);
        let t0 = cvtt(mul((rd32(src + 0x2b4) as i32) as f32, rdf(src + 0x28c)));
        let t1 = cvtt(mul((rd32(src + 0x2b0) as i32) as f32, rdf(src + 0x288)));
        let scr = obj.wrapping_add(SCR_BIAS);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            C_SCALE,
            u32,
            scr,
            0,
            (t1 as f32).to_bits(),
            (t0 as f32).to_bits(),
            0,
            f_c0.to_bits(),
            f_c4.to_bits()
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(C_COMMIT, u32, scr, sc.add(0x40) as u32, b as u32);
        wr32(obj + 0x5520, rd32(a0));
        wr32(obj + 0x5524, rd32(a0 + 4));
        wr32(obj + 0x5528, rd32(a0 + 8));
        wr32(obj + 0x5530, rd32(a1));
        wr32(obj + 0x5534, rd32(a1 + 4));
        wr32(obj + 0x5538, rd32(a1 + 8));
        let done: u32 = lf_checker_rt::callee_thiscall!(C_DONE, u32, obj, a2);
        wr32(obj + 0x4000, 0);
        done
    }
}

lf_checker_rt::export!(fastcall, rw_0061BD70(obj: u32, src: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe { fn2_inner(obj, src, a0, a1, a2) }
});


lf_checker_rt::export!(fastcall, rw_0061BD70(obj: u32, src: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe { fn2_inner(obj, src, a0, a1, a2) }
});
