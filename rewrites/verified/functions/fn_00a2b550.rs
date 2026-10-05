// original: 0x00a2b550 ped_task_flag_dispatch

/// Dispatch on the type bits of a ped-like object: either forward to a
/// handler, or run a spatial query and report whether it found anything.
///
/// `obj_this` (ECX) points to a record whose first word is another record
/// with a matrix at `+0x20`; `obj_a` is the subject with a flags word at
/// `+0x28` and an optional position block at `+0x20`. `arg1`/`arg2` are only
/// forwarded on the early path.
///
/// When `(obj_a[0x28] & 0x3c0) == 0xc0` the call is forwarded to callee 0
/// with `([obj_this], obj_a, arg1, arg2)` and its result is returned.
///
/// Otherwise a query is built on the stack: three floats from the game data
/// at file `0x1b4b320` seed two identical candidate blocks (three position
/// vectors each, then zero words, a `0xffff` marker and zero bytes), and a
/// probe vector is formed from the matrix position plus the float constant
/// at file `0xfe88e8` added to its height. Callee 1 runs the query over the
/// source point (`obj_a[0x20]+0x30`, or `obj_a+0x10` when null). A result of
/// 0 or less means nothing found (returns 0); above 1 means found (returns
/// 1); exactly 1 re-checks through callee 2 and returns 0 only when that
/// answer is `obj_a` itself.
///
/// The original reads its game data through relocated absolute addresses;
/// the rewrite reads the same addresses. Only the low byte of the result
/// is significant.
///
/// Original: 0x00a2b550 (thiscall, three stack words; returns low byte).
lf_checker_rt::export!(thiscall, rw_00a2b550(obj_this: u32, obj_a: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x28;
        const TYPE_MASK: u32 = 0x3c0;
        const FORWARD_TYPE: u32 = 0xc0;
        const POS_BLOCK: u32 = 0x20;
        const MATRIX: u32 = 0x20;
        const QUERY_TAG: u32 = 0x8e;
        const QUERY_MODE: u32 = 2;
        const QUERY_FLAGS: u32 = 4;
        const BLOCK_MARK: u32 = 0xffff;
        const G_POS_X: u32 = 0x1b4b320;
        const G_POS_Y: u32 = 0x1b4b324;
        const G_POS_Z: u32 = 0x1b4b328;
        const K_LIFT: u32 = 0xfe88e8;
        // Frame replica: probe vector at +0x10, query head at +0x20, two
        // candidate blocks at +0x30 and +0x90 (each three vectors, zero
        // words, marker, zero bytes, trailing zero).
        const F_PROBE: usize = 0x10;
        const F_QUERY: usize = 0x20;
        const F_BLOCK_A: usize = 0x30;
        const F_BLOCK_B: usize = 0x90;
        const F_LEN: usize = 0xd4;

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
        unsafe fn g(ph: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(ph) as *const u32).read()) }
        }
        #[inline(always)]
        fn put32(f: &mut [u8], at: usize, v: u32) {
            f[at..at + 4].copy_from_slice(&v.to_le_bytes());
        }
        #[inline(always)]
        fn putf(f: &mut [u8], at: usize, v: f32) {
            put32(f, at, v.to_bits());
        }

        if rd32(obj_a.wrapping_add(FLAGS)) & TYPE_MASK == FORWARD_TYPE {
            return lf_checker_rt::callee_cdecl!(0, u32, rd32(obj_this), obj_a, arg1, arg2);
        }

        let (gx, gy, gz) = (g(G_POS_X), g(G_POS_Y), g(G_POS_Z));
        let mut frame = [0u8; F_LEN];
        for base in [F_BLOCK_A, F_BLOCK_B] {
            for v in 0..3 {
                putf(&mut frame, base + v * 0x10, gx);
                putf(&mut frame, base + v * 0x10 + 4, gy);
                putf(&mut frame, base + v * 0x10 + 8, gz);
            }
            put32(&mut frame, base + 0x30, 0);
            put32(&mut frame, base + 0x34, 0);
            put32(&mut frame, base + 0x38, 0);
            put32(&mut frame, base + 0x3c, BLOCK_MARK);
            frame[base + 0x40] = 0;
            frame[base + 0x42] = 0;
            frame[base + 0x43] = 0;
        }
        // Only the first block has the trailing zero word.
        put32(&mut frame, F_BLOCK_A + 0x50, 0);
        // Query head word stays 0 until the callee answers.
        let fbase = frame.as_mut_ptr() as u32;
        let probe = fbase.wrapping_add(F_PROBE as u32);
        let query = fbase.wrapping_add(F_QUERY as u32);

        let anchor = rd32(obj_a.wrapping_add(POS_BLOCK));
        let src = if anchor != 0 {
            anchor.wrapping_add(0x30)
        } else {
            obj_a.wrapping_add(0x10)
        };
        let target = rd32(obj_this);
        let mat = rd32(target.wrapping_add(MATRIX));
        let px = rdf(mat.wrapping_add(0x30));
        let py = rdf(mat.wrapping_add(0x34));
        let lifted = add(rdf(mat.wrapping_add(0x38)), g(K_LIFT));
        putf(&mut frame, F_PROBE, px);
        putf(&mut frame, F_PROBE + 4, py);
        putf(&mut frame, F_PROBE + 8, lifted);

        let found =
            lf_checker_rt::callee_cdecl!(1, u32, probe, src, target, query, QUERY_TAG, QUERY_MODE, QUERY_FLAGS);
        if (found as i32) <= 0 {
            // The original returns a stack byte it never wrote (always 0).
            return 0;
        }
        if found != 1 {
            return 1;
        }
        let again = lf_checker_rt::callee_cdecl!(2, u32, rd32(query));
        if again == obj_a { 0 } else { 1 }
    }
});
