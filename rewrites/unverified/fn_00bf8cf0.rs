// original: 0x00bf8cf0 task_blend_solve_apply (proposed)

/// Solve a small blend system for a task target and apply it, unless a flag
/// byte selects the short path that only lerps one value.
///
/// `this` carries floats at `+0x18`/`+0x20`/`+0x24`/`+0x28`, a dword at
/// `+0x1c` and the flag byte at `+0x2c`; `a8` is an object passed as the
/// callee context; `aC` points at the target (float at `+0x18`, three floats
/// at `+0x20`); `c` is a blend factor as float bits.
///
/// Behaviour: when bit 1 of the flag byte is set, only the tail runs. On the
/// full path a gather call fills two cells (three words each; the stub also
/// stands in for a fourth hole word after the first cell), a solve call fills
/// a four-word cell, and a dot-product-like sum over the two cells is tested:
/// a strictly negative sum xors the four solve words with a global mask word
/// and stores them back. A build call then combines the cells into a
/// four-word object, a transform call expands that into eleven words, and an
/// apply call takes a fifteen-word block (a zero word, the eleven transform
/// words, three computed floats) plus a global-minus-factor chain scaled by
/// `c` and the `+0x20` floats. The tail lerps the target's `+0x18` float
/// towards `this`'s by `c` and pushes it under a fixed key; that call's
/// answer is the return value on both paths.
///
/// Original: 0x00bf8cf0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00bf8cf0(this: u32, a8: u32, a_target: u32, c: u32) -> u32 {
    unsafe {
        const STR_SET_KEY: u32 = 0x00ebc574;
        const G_SUB: u32 = 0x00fe88e8;
        const G_MASK: u32 = 0x00fe8fa0;
        const THIS_F18: u32 = 0x18;
        const THIS_W1C: u32 = 0x1c;
        const THIS_F20: u32 = 0x20;
        const THIS_F24: u32 = 0x24;
        const THIS_F28: u32 = 0x28;
        const THIS_FLAG: u32 = 0x2c;
        const TGT_F18: u32 = 0x18;
        const C_GATHER: u32 = 1;
        const C_SOLVE: u32 = 2;
        const C_BUILD: u32 = 3;
        const C_XFORM: u32 = 4;
        const C_APPLY: u32 = 5;
        const C_SET: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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

        if rd32(this.wrapping_add(THIS_FLAG)) & 0xff & 2 == 0 {
            let mut cell20 = [0u32; 4];
            let mut cell30 = [0u32; 3];
            lf_checker_rt::callee_thiscall!(
                C_GATHER,
                u32,
                a_target,
                cell20.as_mut_ptr() as u32,
                cell30.as_mut_ptr() as u32
            );
            // cell20[3] is never stored (contract stack_fill 0).
            let mut cell10 = [0u32; 4];
            lf_checker_rt::callee_cdecl!(
                C_SOLVE,
                u32,
                cell10.as_mut_ptr() as u32,
                rd32(this.wrapping_add(THIS_W1C))
            );
            let x2 = f32::from_bits(cell10[0]);
            let x3 = f32::from_bits(cell10[1]);
            let x4 = f32::from_bits(cell10[2]);
            let x5 = f32::from_bits(cell10[3]);
            let e20 = f32::from_bits(cell20[0]);
            let e24 = f32::from_bits(cell20[1]);
            let e28 = f32::from_bits(cell20[2]);
            let ta = mul(x3, e24);
            let tb = mul(x2, e20);
            let mut x1 = add(tb, ta);
            let tc = mul(x4, e28);
            x1 = add(x1, tc);
            let td = mul(f32::from_bits(cell20[3]), x5);
            x1 = add(x1, td);
            if x1 < 0.0 {
                let m = rd32(lf_checker_rt::relocated(G_MASK));
                cell10[0] ^= m;
                cell10[1] ^= m;
                cell10[2] ^= m;
                cell10[3] ^= m;
            }
            // obj40 is written by the build stub and only reread by the
            // stubbed transform call, so its words need no snap.
            let mut obj40 = [0u32; 4];
            lf_checker_rt::callee_thiscall!(
                C_BUILD,
                u32,
                obj40.as_mut_ptr() as u32,
                c,
                cell10.as_mut_ptr() as u32,
                cell20.as_mut_ptr() as u32
            );
            let mut obj54 = [0u32; 11];
            lf_checker_rt::callee_thiscall!(
                C_XFORM,
                u32,
                obj54.as_mut_ptr() as u32,
                obj40.as_mut_ptr() as u32
            );
            let g = rdf(lf_checker_rt::relocated(G_SUB));
            let cf = f32::from_bits(c);
            let f30 = f32::from_bits(cell30[0]);
            let f34 = f32::from_bits(cell30[1]);
            let f38 = f32::from_bits(cell30[2]);
            let s20 = rdf(this.wrapping_add(THIS_F20));
            let s24 = rdf(this.wrapping_add(THIS_F24));
            let s28 = rdf(this.wrapping_add(THIS_F28));
            let mut t5 = sub(g, cf);
            let t2 = mul(f30, cf);
            let t4 = mul(f38, cf);
            let t3 = mul(f34, cf);
            let mut t1 = mul(t5, s20);
            let mut t0 = mul(t5, s24);
            t5 = mul(t5, s28);
            t0 = add(t0, t3);
            t1 = add(t1, t2);
            t5 = add(t5, t4);
            let mut blk = [
                0u32,
                obj54[0],
                obj54[1],
                obj54[2],
                obj54[3],
                obj54[4],
                obj54[5],
                obj54[6],
                obj54[7],
                obj54[8],
                obj54[9],
                obj54[10],
                t1.to_bits(),
                t0.to_bits(),
                t5.to_bits(),
            ];
            lf_checker_rt::callee_thiscall!(C_APPLY, u32, a8, blk.as_mut_ptr() as u32);
        }
        let d = rdf(a_target.wrapping_add(TGT_F18));
        let s = rdf(this.wrapping_add(THIS_F18));
        let cf = f32::from_bits(c);
        let t = mul(sub(d, s), cf);
        let lerp = add(t, s);
        lf_checker_rt::callee_thiscall!(
            C_SET,
            u32,
            a8,
            lf_checker_rt::relocated(STR_SET_KEY),
            lerp.to_bits()
        )
    }
});
