// original: 0x00BF23E0 blend_driver
/// Full blend driver: entry selection plus both row loops.
///
/// Shared prefix registers through callee 1, then selects the fast path
/// (flag 0x400 clear in `obj+0x24`, float argument unequal to the shared
/// threshold, two gate reads through callee 2, nonzero `c`) or the
/// resolve path (two function-table resolutions like the sibling
/// `blend_matrix_rows`). Each path then loops the index byte from 0x50
/// up to `this+0x17`: the fast loop predicates each iteration through
/// callees 7+8 and dispatches two frame shapes through callees 9-11
/// before copying 16 words to the indexed float row; the resolve loop
/// predicates the same way, blends heap rows with scalar
/// single-precision multiply-add chains in the original's exact operand
/// order, and copies 16 result words per iteration. Returns the last
/// sizing/report answer (fast) or row-table pointer (resolve) adjusted
/// with the final counter in the low byte; empty loops return the
/// sizing answer and the row-table pointer respectively.
///
/// Cond: `this` points to a readable object with the count byte at
/// +0x17; `obj` to an object with a function table. The fast working
/// objects expose a float-matrix base at +0x14 (sizing answer) and a
/// row-table holder at +0x0 (report answer); the resolve working
/// objects expose the same at +0x14 and +0x4 through their holders.
/// Stubbed callees leave their frame scratch zero-filled, so the
/// loop math is verified over varied heap inputs with zero frame
/// inputs; each loop predicate answers a scripted per-call sequence
/// shared by both loops.
/// A null `obj` returns 0 (the original returns unobservable entry EAX
/// there, which no rewrite can reproduce; the contract never generates
/// it).
export!(thiscall, rw_aq23_f1(this: u32, obj: u32, c: u32, fbits: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(1, u32, this, obj, c, fbits, 1);
        // The original tests ZF against PF after the threshold compare,
        // which is exactly ordered float equality; NaN takes fast path.
        let need_resolve = (*(obj.wrapping_add(0x24) as *const u32) & 0x400) != 0
            || f32::from_bits(fbits) == *global::<f32>(0xFE8628);
        if need_resolve {
            return path_b_full(this, obj);
        }
        let r1: u32 = callee_thiscall!(2, u32, this);
        if r1 == 0xFFFFFF {
            return path_b_full(this, obj);
        }
        let r2: u32 = callee_thiscall!(2, u32, this);
        if r2 == 0 {
            return r2;
        }
        if c == 0 {
            return r2;
        }
        let sized: u32 = callee_thiscall!(3, u32, obj);
        let rep: u32 = callee_thiscall!(4, u32, obj);
        let ans3 = sized;
        let ans4 = rep;
        // Fast row loop. Frame slots mirror the original's body-relative
        // words; slots the original never stores stay zero, matching the
        // checker's defined stack fill.
        let mut st = [0u32; 64];
        st[6] = this;
        st[7] = ans3;
        st[14] = ans4;
        let count = *(this.wrapping_add(0x17) as *const u8);
        let mut ret = rep;
        let mut idx: u8 = 0x50;
        while idx < count {
            st[4] = idx as u32;
            let a7: u32 = callee_cdecl!(7, u32, st[4], 1);
            let a8: u32 = callee_cdecl!(8, u32, a7 & 0xFFFF);
            // Taken arm: gather the indexed row and dispatch through
            // callees 9-11, then copy 16 frame words to the float row.
            let (dest, a11): (u32, u32);
            if (a8 as u8) != 0 {
                let floatbase = *(ans3.wrapping_add(0x14) as *const u32);
                let rowarray = *(ans4 as *const u32);
                let rowstruct = *((idx as u32)
                    .wrapping_mul(0xE0)
                    .wrapping_add(rowarray)
                    .wrapping_add(0x10)
                    as *const u32);
                let sel = *(rowstruct.wrapping_add(0x14) as *const u16) as u32;
                let zb = idx.wrapping_sub(0x50);
                dest = (idx as u32).wrapping_shl(6).wrapping_add(floatbase);
                let src = sel.wrapping_shl(6).wrapping_add(floatbase);
                // The pushed index word carries the row-table address in
                // its upper bytes (caller leftover the callee ignores
                // past the low byte); replicated exactly since heap
                // addresses are identical on both sides.
                let savedword = (rowstruct & 0xFFFFFF00) | zb as u32;
                st[15] = savedword;
                let frame_a = st.as_mut_ptr().add(60) as u32;
                let _: u32 = callee_thiscall!(9, u32, c, frame_a, savedword);
                let frame_b = st.as_mut_ptr().add(20) as u32;
                let _: u32 =
                    callee_thiscall!(10, u32, this, frame_b, frame_a, fbits, savedword);
                st[32] = *(dest.wrapping_add(0x30) as *const u32);
                st[33] = *(dest.wrapping_add(0x34) as *const u32);
                st[34] = *(dest.wrapping_add(0x38) as *const u32);
                st[35] = *(dest.wrapping_add(0x3C) as *const u32);
                let frame_out = st.as_mut_ptr().add(40) as u32;
                a11 = callee_thiscall!(11, u32, frame_out, frame_b, src);
            } else {
                // Je arm: same dispatch with the alternate frame shape;
                // source floats come from the row table.
                let floatbase = *(ans3.wrapping_add(0x14) as *const u32);
                st[5] = floatbase;
                st[8] = idx as u32;
                let e0 = (idx as u32).wrapping_mul(0xE0);
                st[16] = e0;
                let rowarray = *(ans4 as *const u32);
                let rowstruct =
                    *(e0.wrapping_add(rowarray).wrapping_add(0x10) as *const u32);
                let sel = *(rowstruct.wrapping_add(0x14) as *const u16) as u32;
                let zb = idx.wrapping_sub(0x50);
                let src = sel.wrapping_shl(6).wrapping_add(floatbase);
                let savedword = (rowstruct & 0xFFFFFF00) | zb as u32;
                st[15] = savedword;
                let frame_a = st.as_mut_ptr().add(56) as u32;
                let _: u32 = callee_thiscall!(9, u32, c, frame_a, savedword);
                let frame_b = st.as_mut_ptr().add(20) as u32;
                let _: u32 =
                    callee_thiscall!(10, u32, this, frame_b, frame_a, fbits, savedword);
                st[32] = *(e0.wrapping_add(rowarray).wrapping_add(0x20) as *const u32);
                st[33] = *(e0.wrapping_add(rowarray).wrapping_add(0x24) as *const u32);
                st[34] = *(e0.wrapping_add(rowarray).wrapping_add(0x28) as *const u32);
                st[35] = *(e0.wrapping_add(rowarray).wrapping_add(0x2C) as *const u32);
                let frame_out = st.as_mut_ptr().add(40) as u32;
                a11 = callee_thiscall!(11, u32, frame_out, frame_b, src);
                dest = (idx as u32).wrapping_shl(6).wrapping_add(st[5]);
            }
            // Shared tail: 16-word copy, counter bump, low-byte fixup.
            let dst = dest as *mut u32;
            let sbase = st.as_ptr().add(40);
            let mut k = 0usize;
            while k < 16 {
                *dst.add(k) = *sbase.add(k);
                k += 1;
            }
            idx = idx.wrapping_add(1);
            st[4] = idx as u32;
            ret = (a11 & 0xFFFFFF00) | idx as u32;
        }
        ret
    }
});

/// Resolve path: two table resolutions, then the blending row loop.
unsafe fn path_b_full(this: u32, obj: u32) -> u32 {
    let res_a = {
        // Probe slot 0xA0; null falls back to obj+0x100, else fetch
        // again and dispatch slot 0xE0 (no stack args either way).
        let vt = *(obj as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*(vt.wrapping_add(0xa0) as *const u32) as usize);
        if probe(obj) == 0 {
            *(obj.wrapping_add(0x100) as *const u32)
        } else {
            let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(vt.wrapping_add(0xa0) as *const u32) as usize,
            );
            let mid = fetch(obj);
            let vt2 = *(mid as *const u32);
            let tail: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(vt2.wrapping_add(0xe0) as *const u32) as usize,
            );
            tail(mid)
        }
    };
    let res_b = {
        // Probe slot 0xA0; null falls back to obj+0x100, else fetch
        // again and dispatch slot 0xE0 (no stack args either way).
        let vt = *(obj as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*(vt.wrapping_add(0xa0) as *const u32) as usize);
        if probe(obj) == 0 {
            *(obj.wrapping_add(0x100) as *const u32)
        } else {
            let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(vt.wrapping_add(0xa0) as *const u32) as usize,
            );
            let mid = fetch(obj);
            let vt2 = *(mid as *const u32);
            let tail: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                *(vt2.wrapping_add(0xe0) as *const u32) as usize,
            );
            tail(mid)
        }
    };
    let ptr_p = *(res_b.wrapping_add(4) as *const u32);
    let mut st = [0u32; 64];
    st[6] = this;
    st[7] = res_a;
    st[14] = ptr_p;
    let count = *(this.wrapping_add(0x17) as *const u8);
    let mut ret = ptr_p;
    let mut idx: u8 = 0x50;
    // Scalar FP accumulators (the original's xmm0-xmm7 low lanes;
    // only scalar lanes are ever live) and integer temporaries.
    let (mut x0, mut x1, mut x2, mut x3): (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);
    let (mut x4, mut x5, mut x6, mut x7): (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);
    let (mut eax, mut ecx, mut edx, mut esi): (u32, u32, u32, u32) = (0, 0, 0, 0);
    let mut edi: u32 = 0;
    while idx < count {
        st[4] = idx as u32;
        let a7: u32 = callee_cdecl!(7, u32, st[4], 1);
        let a8: u32 = callee_cdecl!(8, u32, a7 & 0xFFFF);
        // Taken arm: gather, blend through the transpiled chains, copy.
        let dest: u32;
        if (a8 as u8) != 0 {
            let floatbase = *(res_a.wrapping_add(0x14) as *const u32);
            let rowarray = *(ptr_p as *const u32);
            let e0 = (idx as u32).wrapping_mul(0xE0);
            let rowstruct =
                *(e0.wrapping_add(rowarray).wrapping_add(0x10) as *const u32);
            let sel = *(rowstruct.wrapping_add(0x14) as *const u16) as u32;
            let zb = idx.wrapping_sub(0x50);
            edi = (idx as u32).wrapping_shl(6).wrapping_add(floatbase);
            esi = sel.wrapping_shl(6).wrapping_add(floatbase);
            eax = zb as u32;
            dest = edi;
            let frame_c = st.as_mut_ptr().add(40) as u32;
            let a12: u32 = callee_thiscall!(12, u32, this, frame_c, eax);
            eax = a12;
            x1 = *(edi.wrapping_add(0x34) as *const f32);
            x2 = *(edi.wrapping_add(0x38) as *const f32);
            x0 = *(edi.wrapping_add(0x30) as *const f32);
            x3 = f32::from_bits(st[0x28]);
            x4 = f32::from_bits(st[0x29]);
            st[0x35] = core::hint::black_box(x1.to_bits());
            st[0x34] = core::hint::black_box(x0.to_bits());
            st[0x36] = core::hint::black_box(x2.to_bits());
            st[0x8] = core::hint::black_box(x0.to_bits());
            x0 = *(edi.wrapping_add(0x3c) as *const f32);
            st[0x37] = core::hint::black_box(x0.to_bits());
            st[0x5] = core::hint::black_box(x1.to_bits());
            x5 = *(esi.wrapping_add(0x8) as *const f32);
            x7 = *(esi.wrapping_add(0x18) as *const f32);
            x0 = x4;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x10) as *const f32)));
            x1 = x3;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(esi as *const f32)));
            x6 = *(esi.wrapping_add(0x28) as *const f32);
            st[0x10] = core::hint::black_box(x2.to_bits());
            x2 = f32::from_bits(st[0x2a]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x20) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x4) as *const f32)));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            st[0x14] = core::hint::black_box(x1.to_bits());
            x1 = x4;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(esi.wrapping_add(0x14) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) * core::hint::black_box(x7));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x24) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x3));
            x3 = f32::from_bits(st[0x2c]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x2 = core::hint::black_box(core::hint::black_box(x2) * core::hint::black_box(x6));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x2));
            x2 = f32::from_bits(st[0x2e]);
            st[0x15] = core::hint::black_box(x1.to_bits());
            x1 = x3;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(esi as *const f32)));
            st[0x16] = core::hint::black_box(x4.to_bits());
            x4 = f32::from_bits(st[0x2d]);
            x0 = x4;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x10) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x20) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x4) as *const f32)));
            st[0x18] = core::hint::black_box(x1.to_bits());
            x1 = x4;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(esi.wrapping_add(0x14) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x24) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x4 = core::hint::black_box(core::hint::black_box(x4) * core::hint::black_box(x7));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            st[0x19] = core::hint::black_box(x1.to_bits());
            x2 = core::hint::black_box(core::hint::black_box(x2) * core::hint::black_box(x6));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x3));
            x3 = f32::from_bits(st[0x30]);
            x1 = x3;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(esi as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x2));
            x2 = f32::from_bits(st[0x32]);
            st[0x1a] = core::hint::black_box(x4.to_bits());
            x4 = f32::from_bits(st[0x31]);
            x0 = x4;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x10) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x20) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x4) as *const f32)));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            st[0x1c] = core::hint::black_box(x1.to_bits());
            x1 = x4;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(esi.wrapping_add(0x14) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) * core::hint::black_box(x7));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x24) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x3));
            x3 = f32::from_bits(st[0x8]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x2 = core::hint::black_box(core::hint::black_box(x2) * core::hint::black_box(x6));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi as *const f32)));
            st[0x1d] = core::hint::black_box(x1.to_bits());
            x1 = *(esi.wrapping_add(0x10) as *const f32);
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x2));
            x2 = f32::from_bits(st[0x10]);
            x6 = core::hint::black_box(core::hint::black_box(x6) * core::hint::black_box(x2));
            st[0x1e] = core::hint::black_box(x4.to_bits());
            x4 = f32::from_bits(st[0x5]);
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(x4));
            x7 = core::hint::black_box(core::hint::black_box(x7) * core::hint::black_box(x4));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = *(esi.wrapping_add(0x20) as *const f32);
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(x2));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(esi.wrapping_add(0x4) as *const f32)));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(*(esi.wrapping_add(0x30) as *const f32)));
            x7 = core::hint::black_box(core::hint::black_box(x7) + core::hint::black_box(x3));
            st[0x20] = core::hint::black_box(x1.to_bits());
            x1 = *(esi.wrapping_add(0x14) as *const f32);
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(x4));
            x7 = core::hint::black_box(core::hint::black_box(x7) + core::hint::black_box(x6));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = *(esi.wrapping_add(0x24) as *const f32);
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(x2));
            x7 = core::hint::black_box(core::hint::black_box(x7) + core::hint::black_box(*(esi.wrapping_add(0x38) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(*(esi.wrapping_add(0x34) as *const f32)));
        } else {
            // Je arm: register the pair through callee 13, blend the
            // alternate chains. The first loop-2 call leaves one pushed
            // word behind which the next call consumes as its second
            // argument; passed explicitly here.
            let w1 = (a8 & 0xFFFFFF00) | (idx.wrapping_sub(0x50) as u32);
            st[39] = idx as u32;
            st[5] = w1;
            let a13: u32 = callee_thiscall!(13, u32, obj, idx as u32);
            let _: u32 = callee_thiscall!(14, u32, this, a13, w1);
            let e0 = (idx as u32).wrapping_mul(0xE0);
            let floatbase = *(res_a.wrapping_add(0x14) as *const u32);
            st[38] = floatbase;
            let rowarray = *(ptr_p as *const u32);
            let rowstruct =
                *(e0.wrapping_add(rowarray).wrapping_add(0x10) as *const u32);
            let sel = *(rowstruct.wrapping_add(0x14) as *const u16) as u32;
            let src2 = sel.wrapping_shl(6).wrapping_add(floatbase);
            st[8] = src2;
            let frame_c = st.as_mut_ptr().add(40) as u32;
            let _: u32 = callee_thiscall!(12, u32, this, frame_c, w1);
            edi = e0;
            esi = rowarray;
            eax = st[0x8];
            x1 = *(edi.wrapping_add(esi).wrapping_add(0x24) as *const f32);
            x2 = *(edi.wrapping_add(esi).wrapping_add(0x28) as *const f32);
            x0 = *(edi.wrapping_add(esi).wrapping_add(0x20) as *const f32);
            x3 = f32::from_bits(st[0x28]);
            x4 = f32::from_bits(st[0x29]);
            st[0x35] = core::hint::black_box(x1.to_bits());
            st[0x34] = core::hint::black_box(x0.to_bits());
            st[0x36] = core::hint::black_box(x2.to_bits());
            st[0x10] = core::hint::black_box(x0.to_bits());
            x0 = *(edi.wrapping_add(esi).wrapping_add(0x2c) as *const f32);
            st[0x37] = core::hint::black_box(x0.to_bits());
            st[0x5] = core::hint::black_box(x1.to_bits());
            x5 = *(eax.wrapping_add(0x8) as *const f32);
            x7 = *(eax.wrapping_add(0x18) as *const f32);
            x0 = x4;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x10) as *const f32)));
            x1 = x3;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(eax as *const f32)));
            x6 = *(eax.wrapping_add(0x28) as *const f32);
            st[0xf] = core::hint::black_box(x2.to_bits());
            x2 = f32::from_bits(st[0x2a]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x20) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x4) as *const f32)));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            st[0x14] = core::hint::black_box(x1.to_bits());
            x1 = x4;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(eax.wrapping_add(0x14) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) * core::hint::black_box(x7));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x24) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x3));
            x3 = f32::from_bits(st[0x2c]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x2 = core::hint::black_box(core::hint::black_box(x2) * core::hint::black_box(x6));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x2));
            x2 = f32::from_bits(st[0x2e]);
            st[0x15] = core::hint::black_box(x1.to_bits());
            x1 = x3;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(eax as *const f32)));
            st[0x16] = core::hint::black_box(x4.to_bits());
            x4 = f32::from_bits(st[0x2d]);
            x0 = x4;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x10) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x20) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            st[0x18] = core::hint::black_box(x1.to_bits());
            x1 = x4;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(eax.wrapping_add(0x14) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) * core::hint::black_box(x7));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x4) as *const f32)));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x24) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x3));
            x3 = f32::from_bits(st[0x30]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x2 = core::hint::black_box(core::hint::black_box(x2) * core::hint::black_box(x6));
            edi = st[0x27];
            edi = edi.wrapping_shl(6);
            st[0x19] = core::hint::black_box(x1.to_bits());
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x2));
            x2 = f32::from_bits(st[0x32]);
            x1 = x3;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(eax as *const f32)));
            st[0x1a] = core::hint::black_box(x4.to_bits());
            x4 = f32::from_bits(st[0x31]);
            x0 = x4;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x10) as *const f32)));
            edi = edi.wrapping_add(st[0x26]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x20) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x4) as *const f32)));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            st[0x1c] = core::hint::black_box(x1.to_bits());
            x1 = x4;
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(*(eax.wrapping_add(0x14) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) * core::hint::black_box(x7));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x2;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x24) as *const f32)));
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x3));
            x3 = f32::from_bits(st[0x10]);
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x2 = core::hint::black_box(core::hint::black_box(x2) * core::hint::black_box(x6));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax as *const f32)));
            st[0x1d] = core::hint::black_box(x1.to_bits());
            x1 = *(eax.wrapping_add(0x10) as *const f32);
            x4 = core::hint::black_box(core::hint::black_box(x4) + core::hint::black_box(x2));
            x2 = f32::from_bits(st[0xf]);
            x6 = core::hint::black_box(core::hint::black_box(x6) * core::hint::black_box(x2));
            st[0x1e] = core::hint::black_box(x4.to_bits());
            x4 = f32::from_bits(st[0x5]);
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(x4));
            x7 = core::hint::black_box(core::hint::black_box(x7) * core::hint::black_box(x4));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = *(eax.wrapping_add(0x20) as *const f32);
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(x2));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = x3;
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(*(eax.wrapping_add(0x4) as *const f32)));
            x3 = core::hint::black_box(core::hint::black_box(x3) * core::hint::black_box(x5));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(*(eax.wrapping_add(0x30) as *const f32)));
            x7 = core::hint::black_box(core::hint::black_box(x7) + core::hint::black_box(x3));
            st[0x20] = core::hint::black_box(x1.to_bits());
            x1 = *(eax.wrapping_add(0x14) as *const f32);
            x1 = core::hint::black_box(core::hint::black_box(x1) * core::hint::black_box(x4));
            x7 = core::hint::black_box(core::hint::black_box(x7) + core::hint::black_box(x6));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x0 = *(eax.wrapping_add(0x24) as *const f32);
            x0 = core::hint::black_box(core::hint::black_box(x0) * core::hint::black_box(x2));
            x7 = core::hint::black_box(core::hint::black_box(x7) + core::hint::black_box(*(eax.wrapping_add(0x38) as *const f32)));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(x0));
            x1 = core::hint::black_box(core::hint::black_box(x1) + core::hint::black_box(*(eax.wrapping_add(0x34) as *const f32)));
            dest = edi;
        }
        // Shared tail: store the final pair, 16-word copy, counter bump.
        // (Pinned like the transpiled stores so no packed arithmetic forms.)
        st[33] = core::hint::black_box(x1.to_bits());
        st[34] = core::hint::black_box(x7.to_bits());
        let dst = dest as *mut u32;
        let sbase = st.as_ptr().add(20);
        let mut k = 0usize;
        while k < 16 {
            *dst.add(k) = *sbase.add(k);
            k += 1;
        }
        idx = idx.wrapping_add(1);
        st[4] = idx as u32;
        ret = (eax & 0xFFFFFF00) | idx as u32;
    }
    ret
}

