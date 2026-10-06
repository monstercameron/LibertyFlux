// original: 0x008f76c0 input_axis_blend3 (proposed)

/// Blend three axis blocks into one output triple.
///
/// `obj` is the device object, `a1`/`a2` two source blocks and `a3` the
/// 12-byte output. The prep callee (thiscall, a scratch triple) runs first;
/// its third word is the fallback output word. The output is the AND of five
/// mask dwords (two from `a2`, two from `a1`, one from `obj`), the product
/// of five floats in order (`a1+0x410`, `obj+0x48`, `a1+0x41C`, `a2+0x8D4`,
/// `a2+0x8E0`), and a third word selected by the flag byte at `obj+0x452`:
/// when clear it is the prep fallback, otherwise a scaled quotient. The
/// quotient path multiplies `a1+0x500` by two global floats, hands the
/// product to the scale callee in XMM0 (thiscall, ECX = `a1`, compared
/// through the vector register), divides a third global by the callee's
/// float answer, clamps the quotient from below at a fourth global with an
/// ordered-above test (NaN clamps), and multiplies it by the product. Float
/// operation order matches the original exactly.
///
/// Thiscall: object in ECX, three stack words, callee cleans 0xC.
lf_checker_rt::export!(thiscall, rw_008f76c0(obj: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const C_PREP: u32 = 1;
        const C_SCALE: u32 = 2;
        const FLAG_OFF: u32 = 0x452;
        const G_MUL1: u32 = 0xfe8728;
        const G_MUL2: u32 = 0xfe8830;
        const G_DIV: u32 = 0xe83e80;
        const G_MIN: u32 = 0xfe88e8;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let rd = |o: u32| unsafe { (o as *const u32).read_unaligned() };
        let rdf = |o: u32| f32::from_bits(rd(o));
        let gf = |va: u32| f32::from_bits(rd(lf_checker_rt::relocated(va)));
        let mut scratch = [0u32; 3];
        let _: u32 = lf_checker_rt::callee_thiscall!(C_PREP, u32, scratch.as_mut_ptr() as u32);
        let mut p = rdf(a1 + 0x410);
        p = mul(p, rdf(obj + 0x48));
        p = mul(p, rdf(a1 + 0x41c));
        p = mul(p, rdf(a2 + 0x8d4));
        p = mul(p, rdf(a2 + 0x8e0));
        let andv = rd(a2 + 0x8dc) & rd(a2 + 0x8d0) & rd(a1 + 0x418) & rd(a1 + 0x40c)
            & rd(obj + 0x44);
        let (o4, o8) = if ((obj + FLAG_OFF) as *const u8).read() != 0 {
            let mut x = rdf(a1 + 0x500);
            x = mul(x, gf(G_MUL1));
            x = mul(x, gf(G_MUL2));
            let rb: u32 = lf_checker_rt::callee_thiscall!(C_SCALE, u32, a1, x.to_bits());
            let mut q = core::hint::black_box(gf(G_DIV)) / core::hint::black_box(f32::from_bits(rb));
            let m = gf(G_MIN);
            if !(q > m) {
                q = m;
            }
            (mul(q, p), q)
        } else {
            (p, f32::from_bits(scratch[2]))
        };
        (a3 as *mut u32).write_unaligned(andv);
        ((a3 + 4) as *mut u32).write_unaligned(o4.to_bits());
        ((a3 + 8) as *mut u32).write_unaligned(o8.to_bits());
        o8.to_bits()
    }
});
