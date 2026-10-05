// original: 0x00981AA0 audio_voice_table_build (proposed)

/// Build an audio entity's voice table, bounds and per-voice weights.
///
/// `this` points to the entity; `arg` is forwarded to a poll callee. A
/// header-parse callee fills two words through an out pointer: a voice
/// count `n` and a flag word. The flag byte at `+0x152` records whether the
/// flag word is non-zero. Unless the capacity word at `+0x32` is already
/// set, it becomes `n` and a block of `n` eight-byte voice entries is
/// allocated at `+0x2c`. The count word at `+0x30` becomes `n`.
///
/// Two words at `+0x13c`/`+0x14c` are copied from a stack slot the original
/// never wrote; under the contract's zero stack fill they are zero. Six
/// bound words at `+0x130`..`+0x148` start at plus/minus `FLT_MAX`. Then, for
/// each of the `n` voices, the poll callee runs again and a second parse
/// callee fills that voice's two words; the running minima (`+0x130`,
/// `+0x134`, and `+0x138` against zero) and maxima (`+0x140`, `+0x144`, and
/// `+0x148` against zero) fold in. Each fold keeps the old bound only when
/// the comparison is strictly greater, so an unordered (NaN) comparison
/// takes the new value.
///
/// A margin constant (one of two globals, selected by the flag byte) is
/// then subtracted from the minima and added to the maxima; six header
/// words are zeroed; a global word is copied to `+0x1c`. A provably dead
/// re-allocation of the voice block (reaching its call needs a zero
/// capacity with a non-zero count, which the first allocation excludes) is
/// replicated without effect, the count word is stored again, and unless
/// the weight-capacity word at `+0x3a` is set it becomes `n` with a block of
/// `n` four-byte weights allocated at `+0x34`. The marker word at `+0x38`
/// becomes `n`; a global quotient seeds `+0x20`/`+0x24`.
///
/// A second loop then walks the voices: for voice `c` (except the last) it
/// takes the distance between consecutive entries,
/// `sqrt((a0-b0)^2 + (a1-b1)^2)` with the exact subtractions and addition
/// order shown, adds it to a running total kept on the stack, and stores
/// the quotient of a global over the distance scaled by another global into
/// that voice's weight. The total lands at `+0x28`. When the flag byte is
/// set the function returns the count word; otherwise eight fixed slots at
/// `+0x3c`, `+0x7c`, `+0x9c` (bytes), `+0xa4`, `+0xe4` are cleared, a fixed
/// callee runs per slot with constant arguments, its answers land at
/// `+0x104`, and the function returns 8. The marker at `+0x150` is zeroed on
/// every path.
///
/// Original: 0x00981AA0 (thiscall, one stack argument, word return in EAX).
lf_checker_rt::export!(thiscall, rw_00981aa0(this: u32, arg: u32) -> u32 {
    unsafe {
        const POLL: u32 = 1;
        const PARSE_HEAD: u32 = 2;
        const PARSE_SLOT: u32 = 3;
        const ALLOC: u32 = 4;
        const SLOT_FIX: u32 = 5;
        const HEAD_NAME: u32 = 0xE8D8AC;
        const SLOT_NAME: u32 = 0xE8D8B4;
        const FLAG: u32 = 0x152;
        const CAP: u32 = 0x32;
        const BLOCK: u32 = 0x2C;
        const COUNT: u32 = 0x30;
        const WCAP: u32 = 0x3A;
        const WEIGHTS: u32 = 0x34;
        const MIN0: u32 = 0x130;
        const MID_LO: u32 = 0x7F7FFFFF;
        const MID_HI: u32 = 0xFF7FFFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn glob_f(v: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(v).read()) }
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

        let r: u32 = lf_checker_rt::callee_cdecl!(POLL, u32, arg, 1);
        let mut o0 = 0u32;
        let mut o1 = 0u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            PARSE_HEAD,
            u32,
            r,
            lf_checker_rt::relocated(HEAD_NAME),
            &mut o0 as *mut u32 as u32,
            &mut o1 as *mut u32 as u32
        );
        wr8(this + FLAG, if o1 > 0 { 1 } else { 0 });
        let n = o0;
        if rd16(this + CAP) == 0 {
            wr16(this + CAP, n as u16);
            let p = if n == 0 {
                0
            } else {
                lf_checker_rt::callee_cdecl!(ALLOC, u32, n.wrapping_mul(8))
            };
            wr32(this + BLOCK, p);
        }
        wr16(this + COUNT, n as u16);
        // The original copies two words from a stack slot it never wrote;
        // the contract fills uninitialized stack with zero.
        wr32(this + 0x13C, 0);
        wr32(this + 0x14C, 0);
        for off in [0u32, 4, 8] {
            wr32(this + MIN0 + off, MID_LO);
            wr32(this + MIN0 + 0x10 + off, MID_HI);
        }
        let mut acc = 0.0f32;
        for i in 0..n {
            let pr: u32 = lf_checker_rt::callee_cdecl!(POLL, u32, arg, 1);
            let base = rd32(this + BLOCK);
            let p0 = base.wrapping_add(i.wrapping_mul(8));
            let _: u32 = lf_checker_rt::callee_cdecl!(
                PARSE_SLOT,
                u32,
                pr,
                lf_checker_rt::relocated(SLOT_NAME),
                p0,
                p0.wrapping_add(4)
            );
            let x3 = rdf(p0);
            let x2 = rdf(p0 + 4);
            let m0 = rdf(this + MIN0);
            wrf(this + MIN0, if x3 > m0 { m0 } else { x3 });
            let m1 = rdf(this + MIN0 + 4);
            wrf(this + MIN0 + 4, if x2 > m1 { m1 } else { x2 });
            let m2 = rdf(this + MIN0 + 8);
            wrf(this + MIN0 + 8, if 0.0 > m2 { m2 } else { 0.0 });
            let m3 = rdf(this + MIN0 + 0x10);
            wrf(this + MIN0 + 0x10, if m3 > x3 { m3 } else { x3 });
            let m4 = rdf(this + MIN0 + 0x14);
            wrf(this + MIN0 + 0x14, if m4 > x2 { m4 } else { x2 });
            let m5 = rdf(this + MIN0 + 0x18);
            wrf(this + MIN0 + 0x18, if m5 > 0.0 { m5 } else { 0.0 });
        }
        let margin = glob_f(if rd8(this + FLAG) != 0 { 0xFE8C20 } else { 0xFE8B38 });
        wrf(this + MIN0, sub(rdf(this + MIN0), margin));
        wrf(this + MIN0 + 4, sub(rdf(this + MIN0 + 4), margin));
        wrf(this + MIN0 + 0x10, add(rdf(this + MIN0 + 0x10), margin));
        wrf(this + MIN0 + 0x14, add(rdf(this + MIN0 + 0x14), margin));
        for off in [0u32, 4, 0x10, 0x18, 0x0C, 0x14] {
            wr32(this + off, 0);
        }
        wr32(this + 0x1C, lf_checker_rt::global::<u32>(0x1231780).read());
        // Dead re-allocation: reaching the call needs a zero capacity with a
        // non-zero count, which the first allocation above excludes.
        if rd16(this + CAP) == 0 {
            wr16(this + CAP, n as u16);
            if n == 0 {
                wr32(this + BLOCK, 0);
            } else {
                let p: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, n.wrapping_mul(8));
                wr32(this + BLOCK, p);
            }
        }
        wr16(this + COUNT, n as u16);
        if rd16(this + WCAP) == 0 {
            wr16(this + WCAP, n as u16);
            if n == 0 {
                wr32(this + WEIGHTS, 0);
            } else {
                let p: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, n.wrapping_mul(4));
                wr32(this + WEIGHTS, p);
            }
        }
        wr16(this + 0x38, n as u16);
        let rate = glob_f(0x1038A18);
        wr32(this + 0x20, rate.to_bits());
        let quot_base = glob_f(0xFE88E8);
        wr32(this + 0x24, div(quot_base, rate).to_bits());
        let bound = rd16(this + COUNT) as u32;
        if bound != 0 {
            let scale = glob_f(0xFE870C);
            let mut c = 0u32;
            while c < bound {
                if c < bound.wrapping_sub(1) {
                    let a = rd32(this + BLOCK);
                    let b = rd32(this + WEIGHTS);
                    let e0 = c.wrapping_mul(8);
                    let x1 = sub(rdf(a.wrapping_add(e0)), rdf(a.wrapping_add(e0 + 8)));
                    let x0 = sub(rdf(a.wrapping_add(e0 + 4)), rdf(a.wrapping_add(e0 + 12)));
                    let s = add(mul(x0, x0), mul(x1, x1));
                    let dist = s.sqrt();
                    acc = add(dist, acc);
                    let m = mul(dist, scale);
                    wr32(b.wrapping_add(c.wrapping_mul(4)), div(quot_base, m).to_bits());
                }
                c += 1;
            }
        }
        wr32(this + 0x28, acc.to_bits());
        if rd8(this + FLAG) != 0 {
            wr16(this + 0x150, 0);
            return rd16(this + COUNT) as u32;
        }
        for k in 0..8u32 {
            wr32(this + 0x3C + k * 4, 0);
            wr32(this + 0x7C + k * 4, 0);
            wr8(this + 0x9C + k, 1);
            wr32(this + 0xA4 + k * 4, 0);
            wr32(this + 0xE4 + k * 4, 0);
            let sr: u32 = lf_checker_rt::callee_cdecl!(SLOT_FIX, u32, 0xFA0, 0x2EE0);
            wr32(this + 0x104 + k * 4, sr);
        }
        wr16(this + 0x150, 0);
        8
    }
});
