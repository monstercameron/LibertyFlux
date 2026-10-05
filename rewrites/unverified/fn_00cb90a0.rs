// original: 0x00CB90A0 peds_task_90a0 (proposed)

/// Steering blend: build a two-float command from a matrix pair and vectors.
///
/// `this` carries a query slot at `+0x54` and a scale at `+0x18`; `a0`
/// carries a vector record at `+0x20`, an owner at `+0xa80` and a bias at
/// `+0xaa0`. After an id call, bit 1 of the owner's word selects the early
/// route (command `[scale, 0]`) or the measurement route: the query slot
/// returns a point, differenced against the vector record into dx/dy/dz. A
/// zero dx/dy length commands `[0, 0]`; otherwise two callee calls fill a
/// scratch matrix (one word stays the defined stack fill) whose entries
/// combine with dx/dy and each other into two accumulators, normalized by
/// `1/sqrt(len)` unless the length is exactly zero, scaled, and issued with
/// a trailing zero to the notify call, followed by the follow-up call whose
/// answer is returned. Original: thiscall with one stack word.
lf_checker_rt::export!(thiscall, rw_00cb90a0(this: u32, a0: u32) -> u32 {
    unsafe {
        const THIS_SCALE: u32 = 0x18;
        const VT_QUERY: u32 = 0x54;
        const ARG_VEC: u32 = 0x20;
        const ARG_OWNER: u32 = 0xA80;
        const ARG_BIAS: u32 = 0xAA0;
        const OWNER_WORD: u32 = 0x50;
        const SIGN: u32 = 0x8000_0000;
        const CALLEE_ID: u32 = 1;
        const CALLEE_MAT_A: u32 = 3;
        const CALLEE_MAT_B: u32 = 4;
        const CALLEE_NOTIFY: u32 = 5;
        const CALLEE_FOLLOW: u32 = 6;

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

        lf_checker_rt::callee_thiscall!(CALLEE_ID, u32, this, a0);
        let owner = rd32(a0 + ARG_OWNER);
        let mut cmd0: f32;
        let mut cmd1: f32;
        if ((rd32(owner + OWNER_WORD) >> 1) & 1) == 0 {
            cmd0 = rdf(this + THIS_SCALE);
            cmd1 = 0.0;
        } else {
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(this) + VT_QUERY) as usize);
            let rp = query(this);
            let vec = rd32(a0 + ARG_VEC);
            let dx = sub(rdf(rp + 4), rdf(vec + 0x34));
            let dy = sub(rdf(rp), rdf(vec + 0x30));
            let dz = sub(rdf(rp + 8), rdf(vec + 0x38));
            let flat = add(mul(dx, dx), mul(dy, dy));
            if flat == 0.0 {
                cmd0 = 0.0;
                cmd1 = 0.0;
            } else {
                // Scratch matrix: words 0..11 mirror the original's frame
                // slots at the shared ecx both callees take; they fill them
                // in call order (the second overwrites most of the first).
                let mut m = [0u32; 12];
                lf_checker_rt::callee_thiscall!(
                    CALLEE_MAT_A,
                    u32,
                    m.as_mut_ptr() as u32,
                );
                let neg = f32::from_bits(rdf(a0 + ARG_BIAS).to_bits() ^ SIGN);
                lf_checker_rt::callee_thiscall!(
                    CALLEE_MAT_B,
                    u32,
                    m.as_mut_ptr() as u32,
                    neg.to_bits(),
                );
                let mf = |i: usize| f32::from_bits(m[i]);
                let acc4 = add(add(mul(mf(4), dx), mul(mf(0), dy)), mul(mf(8), dz));
                let acc5 = add(add(mul(mf(5), dx), mul(mf(1), dy)), mul(mf(9), dz));
                let len = add(mul(acc5, acc5), mul(acc4, acc4));
                let mult = if len == 0.0 {
                    0.0
                } else {
                    core::hint::black_box(1.0f32) / core::hint::black_box(len.sqrt())
                };
                let scale = rdf(this + THIS_SCALE);
                cmd0 = mul(scale, mul(mult, acc5));
                cmd1 = mul(scale, mul(mult, acc4));
            }
        }
        lf_checker_rt::callee_thiscall!(
            CALLEE_NOTIFY,
            u32,
            owner,
            cmd0.to_bits(),
            cmd1.to_bits(),
            0u32,
        );
        lf_checker_rt::callee_thiscall!(CALLEE_FOLLOW, u32, this, a0)
    }
});
