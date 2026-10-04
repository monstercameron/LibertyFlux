// original: 0x00b0d190 net_position_query_hook (proposed)

/// Build a spatial query around an object's position and fire its hook.
///
/// `obj` points to an object with a vtable pointer at `+0`, a position
/// triple at `+0x10`, and an optional sub-object link at `+0x20`. The query
/// centre comes from the sub-object's triple at `+0x30` when the link is
/// non-null, otherwise from the object's own triple. The function fills a
/// stack descriptor with that centre (once plain, once with the height
/// lowered by `DROP` (0.3) and once raised by `LIFT` (1.5)), three copies of
/// a static triple, and flag words, then asks callee 1 (seven stack
/// arguments: three frame pointers, the object, and the constants 6, 1, 4),
/// which answers 0 or non-zero and delivers a result triple through the
/// frame. On a non-zero answer the result's height is raised back by `DROP`
/// and the object's hook (vtable slot `+8`, thiscall with the object in
/// `ecx` and stack arguments (a pointer to the result triple, 0, 0)) runs.
/// Returns nothing observable.
///
/// Only the position block of the descriptor is observed (eight snapshotted
/// words); the static-triple copies and flag words past it are overwritten
/// by the callee's scripted answer or never read back, so no comparison can
/// see them.
///
/// Original: 0x00b0d190 (cdecl, one stack word, no return value).
lf_checker_rt::export!(cdecl, rw_00b0d190(obj: u32) -> u32 {
    unsafe {
        const POS_LINK: u32 = 0x20;
        const POS_FALLBACK: u32 = 0x10;
        const SUB_POS: u32 = 0x30;
        const VTABLE_HOOK: u32 = 0x08;
        const QUERY_CALLEE: u32 = 1;
        const DROP: f32 = f32::from_bits(0x3E99_999A);
        const LIFT: f32 = f32::from_bits(0x3FC0_0000);
        const STATIC_X: u32 = 0x01B4_B320;
        const STATIC_Y: u32 = 0x01B4_B324;
        const STATIC_Z: u32 = 0x01B4_B328;
        // Descriptor words, relative to its base (callee arg 1).
        const W_PX: usize = 0;
        const W_PY: usize = 1;
        const W_PZ_LO: usize = 2;
        const W_RX: usize = 4;
        const W_RY: usize = 5;
        const W_RZ: usize = 6;
        const W_ZERO: usize = 8;
        const W_ANS: usize = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let link = rd32(obj + POS_LINK);
        let base = if link != 0 {
            link.wrapping_add(SUB_POS)
        } else {
            obj.wrapping_add(POS_FALLBACK)
        };
        let px = rdf(base);
        let py = rdf(base + 4);
        let pz = rdf(base + 8);
        let sx = rdf(lf_checker_rt::relocated(STATIC_X));
        let sy = rdf(lf_checker_rt::relocated(STATIC_Y));
        let sz = rdf(lf_checker_rt::relocated(STATIC_Z));
        let pz_lo = sub(pz, DROP);
        let pz_hi = add(pz, LIFT);
        // Zeroed first: the holes between the set words read as zero.
        let mut st = [0u32; 32];
        st[W_PX] = px.to_bits();
        st[W_PY] = py.to_bits();
        st[W_PZ_LO] = pz_lo.to_bits();
        st[W_RX] = px.to_bits();
        st[W_RY] = py.to_bits();
        st[W_RZ] = pz_hi.to_bits();
        st[W_ZERO] = 0;
        st[W_ANS] = sx.to_bits();
        st[W_ANS + 1] = sy.to_bits();
        st[W_ANS + 2] = sz.to_bits();
        st[W_ANS + 4] = sx.to_bits();
        st[W_ANS + 5] = sy.to_bits();
        st[W_ANS + 6] = sz.to_bits();
        st[W_ANS + 8] = sx.to_bits();
        st[W_ANS + 9] = sy.to_bits();
        st[W_ANS + 10] = sz.to_bits();
        st[24] = 0;
        st[25] = 0;
        st[26] = 0;
        st[27] = 0xFFFF;
        // st[28] holds the trailing zero byte and zero word.
        let p2 = st.as_mut_ptr();
        let p3 = p2.add(W_RX) as u32;
        let p1 = p2.add(W_ZERO) as u32;
        let p2 = p2 as u32;
        let r = lf_checker_rt::callee_cdecl!(QUERY_CALLEE, u32, p3, p2, obj, p1, 6, 1, 4);
        if r == 0 {
            return 0;
        }
        let rx = f32::from_bits(st[W_ANS]);
        let ry = f32::from_bits(st[W_ANS + 1]);
        let rz = add(f32::from_bits(st[W_ANS + 2]), DROP);
        st[W_RX] = rx.to_bits();
        st[W_RY] = ry.to_bits();
        st[W_RZ] = rz.to_bits();
        let p4 = st.as_mut_ptr().add(W_RX) as u32;
        let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(rd32(obj) + VTABLE_HOOK) as usize) };
        hook(obj, p4, 0, 0);
        0
    }
});
