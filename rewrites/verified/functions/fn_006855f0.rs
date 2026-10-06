// original: 0x006855F0 trackfield

/// Sample a scalar field over a computed number of steps, orient each sample
/// through seeded float triplets, and emit the frames to a sink.
///
/// `this` points to the emitter (`+0x00` vtable). `seed` feeds the sampler
/// hooks' context, `count` drives the signed magic-division loop bound,
/// `flags` bit 7 trims the bound and arms the last-step marker while bit 31
/// selects the tail sampler call, and the fourth word is popped but unread.
///
/// Behaviour: compute the bound as the high word of the signed product of
/// `count` with the division magic, adjusted by the sign, minus flag bit 7
/// and one; run that many steps when positive. Each step fetches a float
/// triplet from the sampler (callee 1, thiscall/2); on the first step and
/// on the flagged last step also build a header entry through the factory
/// (callee 3, thiscall/1, vtable slot 1), tag it, hand the triplet to the
/// orient hook (callee 4, thiscall/1, vtable slot 13), notify the sink
/// (callee 5, thiscall/1) and fetch a second triplet (callee 2). Then build
/// the step entry (factory arg 1), tag it from the flag, and normalise each
/// triplet component: an exact zero (either sign) maps to fixed constants,
/// any other value (including NaN) goes through the shaping hooks
/// (callees 6 and 7, each taking and returning one float in XMM0), where
/// each test compares its component against zero (either sign is
/// equal, NaN counts as different). Combine the six shaped values with the exact multiply,
/// add, subtract and sign-flip sequence of the original (units constant and
/// sign mask read from the image), hand the frame to the filter hook
/// (callee 8, thiscall/1) and the emit hook (callee 9, thiscall/1, vtable
/// slot 14), and notify the sink again. The tail returns the sampler's
/// answer when flag bit 31 is set, else the bound itself when the loop never ran and bound minus one after it.
///
/// Comparisons: the bound gate and loop bound are signed (the bound goes
/// negative for small and negative counts); the step-zero test reads an
/// unsigned word; the flag test is equality; the three float tests are
/// exact not-equal (NaN counts as different, either zero as equal).
/// Signalling NaNs never occur in the contract. Thiscall with four stack
/// words.
lf_checker_rt::export!(thiscall, rw_006855F0(this: u32, seed: u32, count: u32, flags: u32, _a4: u32) -> u32 {
    unsafe {
        const DIV_MAGIC: u32 = 0x55555556;
        const UNITS_ADDR: u32 = 0xFE88E8;
        const SIGNMASK_ADDR: u32 = 0xFE8FA0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u32) {
            unsafe { (a as *mut u16).write_unaligned(v as u16) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        unsafe fn factory(this: u32, arg: u32) -> u32 {
            unsafe {
                let make: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(4)) as usize);
                make(this, arg)
            }
        }

        let prod = (DIV_MAGIC as i32 as i64).wrapping_mul(count as i32 as i64);
        let hi = ((prod >> 32) & 0xFFFF_FFFF) as u32;
        let bound = hi.wrapping_add(hi >> 31).wrapping_sub((flags >> 7) & 1).wrapping_sub(1);
        let units = f32::from_bits(rd32(lf_checker_rt::relocated(UNITS_ADDR)));
        let signmask = rd32(lf_checker_rt::relocated(SIGNMASK_ADDR));
        if (bound as i32) <= 0 {
            if (flags & 0x8000_0000) != 0 {
                let mut tail = [0u32; 3];
                let answer: u32 = lf_checker_rt::callee_thiscall!(1, u32, seed, tail.as_mut_ptr() as u32, 4);
                return answer;
            }
            return bound;
        }
        let last = bound.wrapping_sub(1);
        let mut k = 0u32;
        while (k as i32) < (bound as i32) {
            let flag = k == last && (flags & 0x80) != 0;
            let mut fbuf = [0u32; 3];
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, seed, fbuf.as_mut_ptr() as u32, 12);
            let (f0, f1, f2) = (f32::from_bits(fbuf[0]), f32::from_bits(fbuf[1]), f32::from_bits(fbuf[2]));
            let (g0, g1, g2);
            if (k as u16) == 0 || flag {
                let head = factory(this, 0);
                wr8(head.wrapping_add(5), if flag { 5 } else { 0 });
                wr16(head.wrapping_add(6), if flag { 0 } else { k });
                let rbuf = [f0.to_bits(), f1.to_bits(), f2.to_bits()];
                let orient: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(head).wrapping_add(0x34)) as usize);
                orient(head, rbuf.as_ptr() as u32);
                let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, head);
                let mut gbuf = [0u32; 3];
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, seed, gbuf.as_mut_ptr() as u32, 12);
                g0 = f32::from_bits(gbuf[0]);
                g1 = f32::from_bits(gbuf[1]);
                g2 = f32::from_bits(gbuf[2]);
            } else {
                g0 = f0;
                g1 = f1;
                g2 = f2;
            }
            let entry = factory(this, 1);
            wr8(entry.wrapping_add(5), if flag { 6 } else { 1 });
            wr16(entry.wrapping_add(6), if flag { 0 } else { k });
            let (t0, u0v) = if g0 != 0.0 {
                let t: u32 = lf_checker_rt::callee_cdecl!(6, u32, g0.to_bits());
                let u: u32 = lf_checker_rt::callee_cdecl!(7, u32, g0.to_bits());
                (f32::from_bits(t), f32::from_bits(u))
            } else {
                (units, 0.0)
            };
            let (t1, u1) = if g1 != 0.0 {
                let t: u32 = lf_checker_rt::callee_cdecl!(6, u32, g1.to_bits());
                let u: u32 = lf_checker_rt::callee_cdecl!(7, u32, g1.to_bits());
                (f32::from_bits(t), f32::from_bits(u))
            } else {
                (units, 0.0)
            };
            let (xmm4, x6) = if g2 != 0.0 {
                let t: u32 = lf_checker_rt::callee_cdecl!(6, u32, g2.to_bits());
                let u: u32 = lf_checker_rt::callee_cdecl!(7, u32, g2.to_bits());
                (f32::from_bits(t), f32::from_bits(u))
            } else {
                (units, 0.0)
            };
            let m60 = fmul(xmm4, t1);
            let m68 = f32::from_bits(u1.to_bits() ^ signmask);
            let q1 = fmul(u1, t0);
            let m64 = fmul(x6, t1);
            let q2 = fmul(u1, u0v);
            let p3 = fmul(x6, t0);
            let m70 = fsub(fmul(q2, xmm4), p3);
            let m78 = fadd(fmul(q2, x6), fmul(xmm4, t0));
            let m8c = fmul(t1, t0);
            let m84 = fadd(fmul(q1, xmm4), fmul(x6, u0v));
            let m88 = fsub(fmul(q1, x6), fmul(xmm4, u0v));
            let vbuf = [m60.to_bits(), m64.to_bits(), m68.to_bits()];
            let xbuf = [0u32; 4];
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, xbuf.as_ptr() as u32, vbuf.as_ptr() as u32);
            let _ = (m70, m78, m84, m88, m8c);
            let emit: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(entry).wrapping_add(0x38)) as usize);
            emit(entry, xbuf.as_ptr() as u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, entry);
            k = k.wrapping_add(1);
        }
        if (flags & 0x8000_0000) != 0 {
            let mut tail = [0u32; 3];
            let answer: u32 = lf_checker_rt::callee_thiscall!(1, u32, seed, tail.as_mut_ptr() as u32, 4);
            return answer;
        }
        bound.wrapping_sub(1)
    }
});
