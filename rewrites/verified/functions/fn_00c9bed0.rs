// original: 0x00c9bed0 chain_matrix_compose (proposed)

/// Resolve the task record for `index` and compose its matrix chain into `dest`.
///
/// `this` is a task-holder object: `+0x20` holds a pointer to the working
/// 12-dword matrix slot (words at offsets 0,4,8,0x10,0x14,0x18,0x20,0x24,
/// 0x28,0x30,0x34,0x38; the words at 0x0c,0x1c,0x2c are never touched),
/// `+0x100` holds a fallback record used when the virtual probe fails. The
/// probe calls the virtual slot at `+0xa0` on `this`; a null answer selects
/// the fallback, otherwise slot `+0xa0` is called again and slot `+0xe0` on
/// its answer gives the record.
///
/// When the probe yields null, the slot matrix is copied to `dest` (allocating
/// the slot through the two direct callees first when `+0x20` is null).
/// Otherwise `dest` is set to the identity matrix, the record table at
/// `[probe + 4]` is indexed by `index` with a stride of `0xe0` bytes, the
/// looked-up matrix replaces `dest`, and then every following record on the
/// `+0x10` link chain is composed in with `dest = dest * source` in 3-wide
/// row groups plus the translation row. A null slot is allocated the same way
/// at the end, and the final direct callee runs on (`dest`, slot).
///
/// All twelve composed words use the original's exact SSE operand order
/// (pinned through `mul`/`add`), including the mixed orders the compiler
/// scheduled (`S0*D0` beside `D0*S1`). Returns the last copied word on the
/// copy path and the final callee's answer on the compose path (both callers
/// ignore it).
///
/// Original: 0x00c9bed0 (thiscall, two stack words: an integer index and the
/// destination matrix pointer).
lf_checker_rt::export!(thiscall, rw_00c9bed0(this: u32, index: u32, dest: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x20;
        const FALLBACK: u32 = 0x100;
        const VT_PROBE: u32 = 0xa0;
        const VT_RECORD: u32 = 0xe0;
        const REC_BASE: u32 = 0x04;
        const REC_NEXT: u32 = 0x10;
        const REC_WORD: u32 = 0x14;
        const REC_STRIDE: u32 = 0xe0;
        const CALLOC: u32 = 2;
        const CINIT: u32 = 3;
        const CLOOKUP: u32 = 4;
        const CFINAL: u32 = 5;
        const ONE: f32 = 1.0;
        /// The twelve matrix words the function ever reads or writes.
        const WORDS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Call virtual `slot` on `obj` (thiscall, no stack arguments).
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(obj)
            }
        }
        /// The shared probe: null answer selects the `+0x100` fallback,
        /// otherwise the second probe answer's `+0xe0` slot gives the record.
        #[inline(always)]
        unsafe fn probe(this: u32) -> u32 {
            unsafe {
                if vcall(this, VT_PROBE) == 0 {
                    rd32(this.wrapping_add(FALLBACK))
                } else {
                    let again = vcall(this, VT_PROBE);
                    vcall(again, VT_RECORD)
                }
            }
        }
        /// Copy the twelve matrix words from `src` to `dst`.
        #[inline(always)]
        unsafe fn copy12(dst: u32, src: u32) {
            unsafe {
                for off in WORDS {
                    wr32(dst.wrapping_add(off), rd32(src.wrapping_add(off)));
                }
            }
        }
        /// Allocate the `+0x20` slot through the two direct callees.
        #[inline(always)]
        unsafe fn ensure_slot(this: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(CALLOC, u32, this);
                let s = rd32(this.wrapping_add(SLOT));
                lf_checker_rt::callee_thiscall!(CINIT, u32, this.wrapping_add(0x10), s);
            }
        }
        /// Compose one looked-up source matrix `s` into `dest`: twelve words
        /// in the original's exact operand order (see the doc comment).
        #[inline(always)]
        unsafe fn compose(dest: u32, s: u32) {
            unsafe {
                let d0 = rdf(dest);
                let d1 = rdf(dest.wrapping_add(4));
                let d2 = rdf(dest.wrapping_add(8));
                let d4 = rdf(dest.wrapping_add(0x10));
                let d5 = rdf(dest.wrapping_add(0x14));
                let d6 = rdf(dest.wrapping_add(0x18));
                let d8 = rdf(dest.wrapping_add(0x20));
                let d9 = rdf(dest.wrapping_add(0x24));
                let d10 = rdf(dest.wrapping_add(0x28));
                let d12 = rdf(dest.wrapping_add(0x30));
                let d13 = rdf(dest.wrapping_add(0x34));
                let d14 = rdf(dest.wrapping_add(0x38));
                let s0 = rdf(s);
                let s1 = rdf(s.wrapping_add(4));
                let s2 = rdf(s.wrapping_add(8));
                let s4 = rdf(s.wrapping_add(0x10));
                let s5 = rdf(s.wrapping_add(0x14));
                let s6 = rdf(s.wrapping_add(0x18));
                let s8 = rdf(s.wrapping_add(0x20));
                let s9 = rdf(s.wrapping_add(0x24));
                let s10 = rdf(s.wrapping_add(0x28));
                let s12 = rdf(s.wrapping_add(0x30));
                let s13 = rdf(s.wrapping_add(0x34));
                let s14 = rdf(s.wrapping_add(0x38));
                let n0 = add(add(mul(s0, d0), mul(s4, d1)), mul(s8, d2));
                let n1 = add(add(mul(d0, s1), mul(d1, s5)), mul(d2, s9));
                let n2 = add(add(mul(s2, d0), mul(s6, d1)), mul(s10, d2));
                let n4 = add(add(mul(s0, d4), mul(d5, s4)), mul(d6, s8));
                let n5 = add(add(mul(d4, s1), mul(d5, s5)), mul(d6, s9));
                let n6 = add(add(mul(d4, s2), mul(d5, s6)), mul(d6, s10));
                let n8 = add(add(mul(d8, s0), mul(d9, s4)), mul(d10, s8));
                let n9 = add(add(mul(d8, s1), mul(d9, s5)), mul(d10, s9));
                let n10 = add(add(mul(d8, s2), mul(d9, s6)), mul(d10, s10));
                let n12 = add(add(add(mul(d12, s0), mul(d13, s4)), mul(d14, s8)), s12);
                let n13 = add(add(add(mul(d12, s1), mul(d13, s5)), mul(d14, s9)), s13);
                let n14 = add(add(add(mul(d12, s2), mul(d13, s6)), mul(d14, s10)), s14);
                wrf(dest, n0);
                wrf(dest.wrapping_add(4), n1);
                wrf(dest.wrapping_add(8), n2);
                wrf(dest.wrapping_add(0x10), n4);
                wrf(dest.wrapping_add(0x14), n5);
                wrf(dest.wrapping_add(0x18), n6);
                wrf(dest.wrapping_add(0x20), n8);
                wrf(dest.wrapping_add(0x24), n9);
                wrf(dest.wrapping_add(0x28), n10);
                wrf(dest.wrapping_add(0x30), n12);
                wrf(dest.wrapping_add(0x34), n13);
                wrf(dest.wrapping_add(0x38), n14);
            }
        }

        if probe(this) == 0 {
            if rd32(this.wrapping_add(SLOT)) == 0 {
                ensure_slot(this);
            }
            let src = rd32(this.wrapping_add(SLOT));
            copy12(dest, src);
            return rd32(src.wrapping_add(0x38));
        }
        wrf(dest, ONE);
        wr32(dest.wrapping_add(4), 0);
        wr32(dest.wrapping_add(8), 0);
        wr32(dest.wrapping_add(0x10), 0);
        wrf(dest.wrapping_add(0x14), ONE);
        wr32(dest.wrapping_add(0x18), 0);
        wr32(dest.wrapping_add(0x20), 0);
        wr32(dest.wrapping_add(0x24), 0);
        wrf(dest.wrapping_add(0x28), ONE);
        wr32(dest.wrapping_add(0x30), 0);
        wr32(dest.wrapping_add(0x34), 0);
        wr32(dest.wrapping_add(0x38), 0);
        let table = rd32(probe(this).wrapping_add(REC_BASE));
        let first = rd32(table).wrapping_add(index.wrapping_mul(REC_STRIDE));
        let got = lf_checker_rt::callee_thiscall!(CLOOKUP, u32, this, rd16(first.wrapping_add(REC_WORD)));
        copy12(dest, got);
        let mut link = rd32(first.wrapping_add(REC_NEXT));
        while link != 0 {
            let s = lf_checker_rt::callee_thiscall!(CLOOKUP, u32, this, rd16(link.wrapping_add(REC_WORD)));
            compose(dest, s);
            link = rd32(link.wrapping_add(REC_NEXT));
        }
        if rd32(this.wrapping_add(SLOT)) == 0 {
            ensure_slot(this);
        }
        let slot = rd32(this.wrapping_add(SLOT));
        lf_checker_rt::callee_thiscall!(CFINAL, u32, dest, slot)
    }
});
