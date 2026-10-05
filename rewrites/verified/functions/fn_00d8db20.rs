// original: 0x00d8db20 audio_voice_spawn_setup (proposed)

/// Spawn one audio voice from its template: validate the template, derive
/// three direction triples, create the voice through a factory, and copy the
/// template's bytes, words and flags into it.
///
/// Thiscall with the template in `ecx`, no stack words. It calls the gate as
/// `gate(template[0x10], global)` and returns 0 unless the answer is nonzero.
/// It scales the three signed bytes at `+0x43/0x44/0x45` by the constant at
/// `SCALE_ADDR`, normalizes the resulting vector unless it is zero (the
/// zero/NaN test is an unordered-compare whose jump is taken for anything
/// but IEEE-equal), derives a second normalization from the negated first
/// component, and hands the triple
/// `(x3b * x3, x3b * x6, x3b * 0.0)` to the factory virtual call
/// `factory(word, 1, triple, 1)` together with the template word. A null
/// factory answer returns 0; otherwise the voice is set up: masked flag
/// words, five bit-field merges of the template word at `+0x30` into the
/// voice word at `+0x118`, conditional flag ors, three thiscalls, an optional
/// probe/program pair gated on bit 1 of the template word (otherwise a default
/// byte), a finalize call, a second virtual call, and the voice pointer as
/// the return value. Float operation order, including the packed square roots
/// (scalar-equivalent: their upper lanes are all zero) and the
/// compare-jump-not-equal idiom, is the original's.
///
/// The template floats at `+0/4/8` are saved to dead frame slots, and the
/// second half of the float derivation feeds only dead stores; both are
/// reproduced faithfully but observed only through the factory triple
/// snapshot. One frame word is read before it is ever written; it holds the
/// defined stack fill (zero).
///
/// Original: 0x00d8db20 (thiscall, template in ecx; returns voice or null).
lf_checker_rt::export!(thiscall, rw_00d8db20(this: u32) -> u32 {
    unsafe {
        const FACTORY: u32 = 1; // indirect, called through the vtable
        const GATE: u32 = 2;
        const SETUP0: u32 = 3;
        const SETUP1: u32 = 4;
        const SETUP2: u32 = 5;
        const PROBE: u32 = 6;
        const PROGRAM: u32 = 7;
        const FINALIZE: u32 = 8;
        const START: u32 = 9; // indirect, called through the vtable
        const GLOBAL_A: u32 = 0x012b4138;
        const FACTORY_HOLDER: u32 = 0x0166d9fc;
        const SCALE_ADDR: u32 = 0x00fe870c;
        const ONE_ADDR: u32 = 0x00fe88e8;
        const SIGNMASK_ADDR: u32 = 0x00fe8fa0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sqrtx(x: f32) -> f32 {
            core::hint::black_box(x).sqrt()
        }
        #[inline(always)]
        fn norm_recip(s: f32, one: f32) -> f32 {
            // ucomiss(s, 0) + jp-taken-iff-not-equal: NaN takes the sqrt path.
            if s != 0.0 {
                div(one, sqrtx(s))
            } else {
                0.0
            }
        }

        let word = rd32(edi + 0x10) & 0xffff;
        let ga = rd32(lf_checker_rt::relocated(GLOBAL_A));
        let r: u32 = lf_checker_rt::callee_cdecl!(GATE, u32, word, ga);
        if (r as u8) == 0 {
            return 0;
        }
        let c0 = rdf(lf_checker_rt::relocated(SCALE_ADDR));
        let one = rdf(lf_checker_rt::relocated(ONE_ADDR));
        let b43 = ((edi + 0x43) as *const i8).read() as f32;
        let b44 = ((edi + 0x44) as *const i8).read() as f32;
        let b45 = ((edi + 0x45) as *const i8).read() as f32;
        let x6 = mul(b43, c0);
        let x3 = mul(b44, c0);
        let x5 = mul(b45, c0);
        let l58 = mul(x3, x3);
        let s1 = add(add(mul(x6, x6), l58), mul(x5, x5));
        let x4 = norm_recip(s1, one);
        let l70 = mul(x4, x6);
        let _l30 = l70;
        let mask = rd32(lf_checker_rt::relocated(SIGNMASK_ADDR));
        let x6n = f32::from_bits(x6.to_bits() ^ mask);
        let l74 = mul(x4, x3);
        let _l2c = l74;
        let x4b = mul(x4, x5);
        let _l28 = x4b;
        let x0b2 = add(mul(x6n, x6n), l58);
        let x3b = norm_recip(x0b2, one);
        // Factory triple snapshot.
        let t0 = mul(x3b, x3);
        let t1 = mul(x3b, x6n);
        let t2 = mul(x3b, 0.0);
        let mut l40 = [t0.to_bits(), t1.to_bits(), t2.to_bits()];
        // Dead second half of the derivation, reproduced faithfully.
        let mut x6d = mul(t1, x4b);
        let mut x1d = mul(t1, l70);
        let mut x0d = mul(t2, l74);
        let mut x3d = mul(t2, l70);
        x6d = sub(x6d, x0d);
        x0d = mul(t0, x4b);
        let mut x5d = mul(t0, l74);
        x5d = sub(x5d, x1d);
        x3d = sub(x3d, x0d);
        let s3 = add(add(mul(x3d, x3d), mul(x6d, x6d)), mul(x5d, x5d));
        // jnp: skip (keep zero) iff equal; NaN takes the sqrt path.
        let x2 = if s3 == 0.0 { 0.0 } else { div(one, sqrtx(s3)) };
        let _l20 = mul(x6d, x2);
        let _l1c = mul(x3d, x2);
        let _l18 = mul(x5d, x2);
        let _dead_vec = (rdf(edi), rdf(edi + 4), rdf(edi + 8), 0.0f32);

        let holder = rd32(lf_checker_rt::relocated(FACTORY_HOLDER));
        let vtab = rd32(holder);
        let fac: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtab + 4) as usize);
        let esi = fac(holder, word, 1, l40.as_mut_ptr() as u32, 1);
        let _ = FACTORY;
        if esi == 0 {
            return 0;
        }
        let e = rd32(esi + 0x28);
        if e & 0x7c00 != 0x0c00 {
            wr32(esi + 0x28, (e & 0xffff8bff) | 0x800);
        }
        wr32(esi + 0xf50, 0);
        wr32(esi + 0xdcc, rd32(edi + 0xc));
        wr32(esi + 0xdd0, rd32(edi + 0x38));
        ((esi + 0xf15) as *mut u8).write(((esi + 0xf15) as *const u8).read() & 0xfd);
        let fb = ((esi + 0xf16) as *const u8).read();
        ((esi + 0xf16) as *mut u8).write((fb & 0xfb) | 2);
        wr32(esi + 0x12d0, 1);
        let w = rd32(edi + 0x30) & 0xffff;
        let e1 = rd32(esi + 0x118);
        wr32(esi + 0x118, e1 ^ ((((w << 4) ^ e1) & 0x40)));
        let e2 = rd32(esi + 0x118);
        wr32(esi + 0x118, (((w << 4) ^ e2) & 0x80) ^ e2);
        let e3 = rd32(esi + 0x118);
        wr32(esi + 0x118, (((w << 8) ^ e3) & 0x1000) ^ e3);
        let e4 = rd32(esi + 0x118);
        wr32(esi + 0x118, (((w << 3) ^ e4) & 0x100) ^ e4);
        let e5 = rd32(esi + 0x118);
        wr32(esi + 0x118, (((w << 3) ^ e5) & 0x200) ^ e5);
        if ((edi + 0x30) as *const u8).read() & 0x80 != 0 {
            ((esi + 0xf1a) as *mut u8).write(((esi + 0xf1a) as *const u8).read() | 0x40);
        }
        if ((edi + 0x31) as *const u8).read() & 1 != 0 {
            wr32(esi + 0xdcc, rd32(esi + 0xdcc) | 0x20000);
        }
        if rd32(edi + 0x30) & 0x200 != 0 {
            wr32(esi + 0xdcc, rd32(esi + 0xdcc) | 0x80000);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(SETUP0, u32, esi);
        ((esi + 0x1072) as *mut u8).write(((edi + 0x42) as *const u8).read());
        ((esi + 0xf94) as *mut u8).write(((edi + 0x32) as *const u8).read());
        ((esi + 0xf95) as *mut u8).write(((edi + 0x33) as *const u8).read());
        ((esi + 0xf96) as *mut u8).write(((edi + 0x34) as *const u8).read());
        ((esi + 0xf97) as *mut u8).write(((edi + 0x35) as *const u8).read());
        let _: u32 = lf_checker_rt::callee_thiscall!(SETUP1, u32, esi, rd32(edi + 0x3c));
        let _: u32 = lf_checker_rt::callee_thiscall!(SETUP2, u32, esi);
        ((esi + 0xf1b) as *mut u8).write(((esi + 0xf1b) as *const u8).read() | 8);
        if ((edi + 0x30) as *const u8).read() & 2 != 0 {
            let mut l24 = 0u32;
            let mut l84 = 0xffffffffu32;
            let pr: u32 = lf_checker_rt::callee_cdecl!(
                PROBE, u32, &mut l24 as *mut u32 as u32,
                &mut l84 as *mut u32 as u32, 0x40000000
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(PROGRAM, u32, esi, pr, l84, 0xffffffff);
        } else {
            ((esi + 0x41) as *mut u8).write(2);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(FINALIZE, u32, esi, 0);
        let vtab2 = rd32(esi);
        let st: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtab2 + 0x194) as usize);
        let _: u32 = st(esi);
        let _ = START;
        esi
    }
});
