// original: 0x00D401F0 task_classify_and_solve_float (proposed)

/// Classify the ped's task, then solve one float from its position block and
/// combine it into a frame block for the combiner helper.
///
/// `obj` points to the ped. Virtual slot 7 on the anchor object at `+0x224`
/// yields a tag that the classifier (thiscall, no stack arguments) turns
/// into a class code; class `0x5D` returns 0 at once (the original's
/// `(an instruction of the original)` clears only the low byte, and the code equals `0x5D`, so the
/// exit value is always 0 there).
///
/// Otherwise the position block at `+0x20` supplies X (`+0x10`), Y (`+0x14`)
/// and three more floats (`+0x30`, `+0x34`, `+0x38`). The reducer callee
/// takes `-X` and `Y` widened to doubles in the low halves of XMM0 and XMM1
/// and answers a double in XMM0, narrowed back to the seed float. Two
/// shaping callees each take the seed in XMM0 and answer a float in XMM0;
/// the seed, the shapes and three file constants (scale, bias, trim) form
/// the rows `pa - g*scale`, `h*scale + pb`, `pc + bias`, handed to the
/// 8-argument combiner as two frame pointers plus the constant words
/// `0x3E0A3D71` and `0x8E` and five zero words. The exit value is the
/// combiner's answer with its low byte replaced by the zero/non-zero status
/// (the original's `(an instruction of the original); setne al`).
///
/// A null position block loads the fallback word at `+0x1C` and then faults
/// reading `[0x30]`, exactly like the original. All class comparisons are
/// equalities. Original: stdcall/1, returns `eax`.
lf_checker_rt::export!(stdcall, rw_00d401f0(obj: u32) -> u32 {
    unsafe {
        const ANCHOR_OFF: u32 = 0x224;
        const SLOT_CLASS: u32 = 0x1c;
        const EXIT_CLASS: u32 = 0x5d;
        const POS_LINK: u32 = 0x20;
        const POS_X: u32 = 0x10;
        const POS_Y: u32 = 0x14;
        const POS_A: u32 = 0x30;
        const POS_B: u32 = 0x34;
        const POS_C: u32 = 0x38;
        const NULL_FALLBACK_OFF: u32 = 0x1c;
        const K_SCALE: u32 = 0x00ee_3ed8;
        const K_BIAS: u32 = 0x00fe_8898;
        const K_TRIM: u32 = 0x00fe_88e8;
        const COMBINE_CONST: u32 = 0x3e0a_3d71;
        const COMBINE_MODE: u32 = 0x8e;
        const SIGN: u32 = 0x8000_0000;
        const C_CLASSIFY: u32 = 1;
        const C_REDUCE: u32 = 2;
        const C_SHAPE1: u32 = 3;
        const C_SHAPE2: u32 = 4;
        const C_COMBINE: u32 = 5;

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

        let anchor = rd32(obj.wrapping_add(ANCHOR_OFF));
        let slot = rd32(rd32(anchor).wrapping_add(SLOT_CLASS));
        let tag_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let tag = tag_of(anchor);
        let code: u32 = lf_checker_rt::callee_thiscall!(C_CLASSIFY, u32, tag);
        if code == EXIT_CLASS {
            return code & 0xFFFF_FF00;
        }
        let pos = rd32(obj.wrapping_add(POS_LINK));
        let seed: f32 = if pos == 0 {
            let _fallback = rdf(obj.wrapping_add(NULL_FALLBACK_OFF));
            f32::from_bits(rd32(pos.wrapping_add(POS_A)))
        } else {
            let nx = f32::from_bits(rdf(pos.wrapping_add(POS_X)).to_bits() ^ SIGN);
            let dy = (rdf(pos.wrapping_add(POS_Y)) as f64).to_bits();
            let dx = (nx as f64).to_bits();
            let ans: u64 = lf_checker_rt::callee_cdecl!(
                C_REDUCE, u64,
                dx as u32, (dx >> 32) as u32, dy as u32, (dy >> 32) as u32,
            );
            f64::from_bits(ans) as f32
        };
        let pa = rdf(pos.wrapping_add(POS_A));
        let pb = rdf(pos.wrapping_add(POS_B));
        let pc = rdf(pos.wrapping_add(POS_C));
        let g = f32::from_bits(lf_checker_rt::callee_cdecl!(C_SHAPE1, u32, seed.to_bits(),));
        let k_scale = f32::from_bits(rd32(lf_checker_rt::relocated(K_SCALE)));
        let row = sub(pa, mul(g, k_scale));
        let h = f32::from_bits(lf_checker_rt::callee_cdecl!(C_SHAPE2, u32, seed.to_bits(),));
        let k_bias = f32::from_bits(rd32(lf_checker_rt::relocated(K_BIAS)));
        let k_trim = f32::from_bits(rd32(lf_checker_rt::relocated(K_TRIM)));
        let top = add(pc, k_bias);
        let mid = add(mul(h, k_scale), pb);
        let mut frame = [
            row.to_bits(),
            mid.to_bits(),
            top.to_bits(),
            row.to_bits(),
            mid.to_bits(),
            sub(top, k_trim).to_bits(),
        ];
        let p_first = frame.as_mut_ptr() as u32;
        let p_second = frame.as_mut_ptr().wrapping_add(3) as u32;
        let ans: u32 = lf_checker_rt::callee_cdecl!(
            C_COMBINE, u32, p_first, p_second, COMBINE_CONST, 0, COMBINE_MODE, 0, 0, 0,
        );
        (ans & 0xFFFF_FF00) | u32::from(ans != 0)
    }
});
