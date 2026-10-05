// original: 0x00980CF0 audio_entity_positional_update (proposed)

/// Update an audio entity's positional gain from a voice-table lookup.
///
/// `this` points to the entity. The update runs only when four gates pass:
/// the audio-stop flag (`GATE_STOP`) is not 1, the two generation counters
/// (`GATE_GEN_A/B`) agree, the audio state (`GATE_STATE`) is not `0x12`, and
/// the entity's voice-table link at `+0x0c` is non-null. Otherwise nothing
/// happens (the original still runs its stack-cookie check, which the
/// contract intercepts as a no-effect call).
///
/// On the main path a position callee fills five words (x, y, z plus two
/// more) through an out pointer. The distance from (x, y, z) to the entity's
/// stored position (`POS_X/Y/Z`) is formed as
/// `sqrt((dy*dy + dx*dx) + dz*dz)`, scaled by `DIST_K1`, additionally by
/// `DIST_K2` unless the band flag byte at `+0x12a` is zero (the original
/// tests that byte several instructions ahead and relies on the SSE
/// arithmetic leaving the flags alone), and divided by `DIST_K3`. The
/// original then computes a minimum of that distance against `LIMIT_K` into
/// a stack slot that is overwritten before any read; the rewrite omits that
/// dead store. A signed counter (`RATE_SRC`) is converted exactly to double
/// (with the `2^32` bias table at `RATE_TAB` for negative values), narrowed
/// to float, and divides `LIMIT_K`; the quotient is sent twice to a
/// parameter callee for the sub-object at `+0x44`.
///
/// A gain callee then maps the original's saved entry EDI (read as a float) to a first gain `r1`. One
/// of two post chains is selected by the flag byte at `+0x219` of the object
/// at `+0x120`: each chain runs a shaping callee twice (both times on `r1`,
/// producing `r2` then a dead `r3`) and a second shaper once on `r2`
/// (producing `r4`). A voice index byte and table selector byte (at `+4` and
/// `+0x40` of the voice-table link) choose a send target through the index
/// table: `mult * index + table[sel * SEL_STRIDE + SEL_BASE]`, or null when
/// the index is `0xff`; `r4` is sent there. `r2` is then run through an
/// SSE shaping callee (float in and out of XMM0), scaled by `GAIN_K1` and
/// `GAIN_K2` in that order, truncated to int with `cvttss2si` semantics
/// (out-of-range and NaN give `0x80000000`), and sent as an integer to the
/// same target. A 76-byte struct is filled by a callee, patched (byte
/// `0x2d` gains bits `0x0a`, words 0 and 4 become `r4` and the truncated
/// int), and emitted with a handle from the voice-table link. Finally out
/// words 1..4 become the entity's new stored position. The function returns
/// nothing.
///
/// Original: 0x00980CF0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00980cf0(this: u32) -> u32 {
    unsafe {
        const GATE_STOP: u32 = 0x11F7060;
        const GATE_GEN_A: u32 = 0x12088B4;
        const GATE_GEN_B: u32 = 0xF1C040;
        const GATE_STATE: u32 = 0x1037720;
        const LINK: u32 = 0x0C;
        const POS_X: u32 = 0x60;
        const POS_Y: u32 = 0x64;
        const POS_Z: u32 = 0x68;
        const BAND_FLAG: u32 = 0x12A;
        const DIST_K1: u32 = 0x11735C0;
        const DIST_K2: u32 = 0xFE8830;
        const DIST_K3: u32 = 0x103896C;
        const LIMIT_K: u32 = 0xFE88E8;
        const RATE_SRC: u32 = 0x1038970;
        const RATE_TAB: u32 = 0xFE8F50;
        const SUB_OBJ: u32 = 0x44;
        const GAIN_ARG: u32 = 0x11735B4;
        const FLAG_OBJ: u32 = 0x120;
        const FLAG_OFF: u32 = 0x219;
        const SEL_STRIDE: u32 = 0x6F40;
        const SEL_BASE: u32 = 0x6F14;
        const MULT: u32 = 0x115D968;
        const TABLE: u32 = 0x115D988;
        const GAIN_K1: u32 = 0xE78588;
        const GAIN_K2: u32 = 0xE7858C;
        const STRUCT_FLAG: u32 = 0x2D;
        const POS_CALLEE: u32 = 1;
        const PARAM: u32 = 2;
        const GAIN: u32 = 3;
        const SHAPE: u32 = 4;
        const SHAPE2: u32 = 5;
        const SEND_F: u32 = 6;
        const SSEMAP: u32 = 7;
        const SEND_I: u32 = 8;
        const FILL: u32 = 9;
        const EMIT: u32 = 10;
        const COOKIE: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        /// Truncate exactly like `cvttss2si`: NaN, infinite and
        /// out-of-range inputs give `0x80000000`, the rest truncate toward
        /// zero (Rust's `as` would saturate instead).
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                -2147483648i32
            } else {
                x as i32
            }
        }

        if lf_checker_rt::global::<u32>(GATE_STOP).read() == 1 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0;
        }
        if lf_checker_rt::global::<u32>(GATE_GEN_A).read()
            != lf_checker_rt::global::<u32>(GATE_GEN_B).read()
        {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0;
        }
        if lf_checker_rt::global::<u32>(GATE_STATE).read() == 0x12 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0;
        }
        let link = rd32(this + LINK);
        if link == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return 0;
        }

        let mut out = [0u32; 5];
        let _: u32 =
            lf_checker_rt::callee_thiscall!(POS_CALLEE, u32, this, out.as_mut_ptr() as u32);
        let skip_band = rd8(this + BAND_FLAG) == 0;
        let dx = sub(f32::from_bits(out[0]), rdf(this + POS_X));
        let dy = sub(f32::from_bits(out[1]), rdf(this + POS_Y));
        let dz = sub(f32::from_bits(out[2]), rdf(this + POS_Z));
        let sum = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let mut d = sum.sqrt();
        d = mul(d, glob_f(DIST_K1));
        if !skip_band {
            d = mul(d, glob_f(DIST_K2));
        }
        d = div(d, glob_f(DIST_K3));
        let _ = d;
        let n = lf_checker_rt::global::<i32>(RATE_SRC).read();
        let bias =
            (lf_checker_rt::global::<f64>(RATE_TAB + ((n as u32 >> 31) * 8))).read();
        let rate = (core::hint::black_box(n as f64) + core::hint::black_box(bias)) as f32;
        let q = div(glob_f(LIMIT_K), rate);
        let sub_obj = this + SUB_OBJ;
        // The original also holds the sub-object in ECX for this call, but the
        // cdecl stub compares no registers, so only the two stack arguments
        // are passed (and compared).
        let _: u32 = lf_checker_rt::callee_cdecl!(PARAM, u32, q.to_bits(), q.to_bits());
        let gain_arg = lf_checker_rt::global::<u32>(GAIN_ARG).read();
        // The gain callee's first argument is the word where the original
        // pushed (and has not yet overwritten) its caller's EDI: entry EDI,
        // fixed by the contract.
        const ENTRY_EDI_BITS: u32 = 0x40400000;
        let r1: u32 =
            lf_checker_rt::callee_thiscall!(GAIN, u32, sub_obj, ENTRY_EDI_BITS, gain_arg);
        let flag = rd8(rd32(this + FLAG_OBJ) + FLAG_OFF);
        let (c1, c2) = if flag != 0 {
            (0x12314A4u32, 0x1231424u32)
        } else {
            (0x12313F0u32, 0x12313C8u32)
        };
        let r2: u32 =
            lf_checker_rt::callee_thiscall!(SHAPE, u32, lf_checker_rt::relocated(c1), r1);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(SHAPE, u32, lf_checker_rt::relocated(c2), r1);
        let r4: u32 =
            lf_checker_rt::callee_stdcall!(SHAPE2, u32, r2);

        let mult = lf_checker_rt::global::<u32>(MULT).read();
        let tbase = lf_checker_rt::global::<u32>(TABLE).read();
        let index = rd8(link + 4) as u32;
        let target = if index == 0xFF {
            0
        } else {
            let sel = rd8(link + 0x40) as u32;
            mult.wrapping_mul(index).wrapping_add(rd32(
                tbase
                    .wrapping_add(sel.wrapping_mul(SEL_STRIDE))
                    .wrapping_add(SEL_BASE),
            ))
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(SEND_F, u32, target, r4);
        let r5: u32 = lf_checker_rt::callee_cdecl!(SSEMAP, u32, r2);
        let scaled = mul(mul(f32::from_bits(r5), glob_f(GAIN_K1)), glob_f(GAIN_K2));
        let ival = cvtt(scaled);
        let _: u32 = lf_checker_rt::callee_thiscall!(SEND_I, u32, target, ival as u32);

        let mut buf = [0u32; 16];
        let _: u32 =
            lf_checker_rt::callee_thiscall!(FILL, u32, buf.as_mut_ptr() as u32, 0x4C);
        let flag_at = (buf.as_mut_ptr() as *mut u8).add(STRUCT_FLAG as usize);
        flag_at.write(flag_at.read() | 0x0A);
        buf[3] = r4;
        buf[7] = ival as u32;
        let handle = rd32(link + 0xA4);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(EMIT, u32, handle, handle, buf.as_ptr() as u32);

        wr32(this + POS_X, out[1]);
        wr32(this + POS_Y, out[2]);
        wr32(this + POS_Z, out[3]);
        wr32(this + POS_Z + 4, out[4]);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        0
    }
});
